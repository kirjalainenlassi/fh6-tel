use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, Manager};
use tokio::net::UdpSocket;

use crate::{
    db,
    parser,
    session::SessionAction,
    AppState,
};

pub async fn run(app: AppHandle, port: u16) {
    let addr = format!("0.0.0.0:{port}");
    let socket = match UdpSocket::bind(&addr).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[udp] failed to bind {addr}: {e}");
            let _ = app.emit("udp_bind_failed", format!("Cannot bind port {port}: {e}"));
            return;
        }
    };
    println!("[udp] listening on {addr}");

    let mut buf = vec![0u8; 1024];
    let mut prev_race_on = false;

    loop {
        let (len, _) = match socket.recv_from(&mut buf).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[udp] recv error: {e}");
                continue;
            }
        };

        let raw = &buf[..len];
        let pkt = match parser::parse(raw) {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Always emit live data regardless of session state
        let _ = app.emit("telemetry_tick", &pkt);

        // Session management
        let state = app.state::<AppState>();
        handle_session(&app, &state, &pkt, raw, prev_race_on);
        prev_race_on = pkt.is_race_on;
    }
}

fn handle_session(app: &AppHandle, state: &AppState, pkt: &parser::TelemetryPacket, raw: &[u8], prev_race_on: bool) {
    let mut sm = state.session_manager.lock().unwrap();
    let db = state.db.lock().unwrap();

    // Apply race transition before inserting so the opening packet is captured
    let action = sm.on_race_on_change(prev_race_on, pkt.is_race_on, pkt.car_ordinal, pkt.car_pi);

    match action {
        SessionAction::Open { car_ordinal, car_pi } => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            match db::open_session(&db, now, car_ordinal, car_pi) {
                Ok(id) => {
                    sm.set_active_id(Some(id));
                    println!("[session] opened #{id}");
                }
                Err(e) => {
                    eprintln!("[session] open error: {e}");
                    let _ = app.emit("session_error", format!("Failed to open session: {e}"));
                }
            }
        }
        SessionAction::Close { best_lap } => {
            if let Some(id) = sm.active_session_id() {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64;
                if let Err(e) = db::close_session(&db, id, now, best_lap) {
                    eprintln!("[session] close error: {e}");
                    let _ = app.emit("session_error", format!("Failed to close session: {e}"));
                } else {
                    println!("[session] closed #{id}");
                }
            }
            sm.set_active_id(None);
        }
        SessionAction::None => {}
    }

    if let Some(session_id) = sm.active_session_id() {
        sm.update_best_lap(pkt.best_lap);
        if let Err(e) = db::insert_packet(&db, session_id, pkt.timestamp_ms, raw) {
            eprintln!("[session] insert error: {e}");
            let _ = app.emit("session_error", format!("Failed to write telemetry: {e}"));
        }
    }
}
