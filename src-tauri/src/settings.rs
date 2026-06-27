use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub port: u16,
    pub use_mph: bool,
    pub tire_temp_cold: f32,
    pub tire_temp_optimal: f32,
    pub tire_temp_hot: f32,
    pub auto_record: bool,
    #[serde(default = "Settings::default_theme")]
    pub theme: String,

    // ── Track map (all serde-defaulted so old settings.json keeps loading) ──
    #[serde(default)]
    pub map_enabled: bool,
    #[serde(default)]
    pub map_override: bool,
    #[serde(default)]
    pub map_tile_url: String,
    #[serde(default)]
    pub map_min_zoom: i32,
    #[serde(default = "Settings::default_map_max_zoom")]
    pub map_max_zoom: i32,
    #[serde(default = "Settings::default_map_tile_size")]
    pub map_tile_size: i32,
    /// Two calibration reference points mapping game world (X, Z) to
    /// full-resolution map pixels (X, Y). Calibration is "unset" when A == B.
    #[serde(default)]
    pub map_cal_a_world: [f64; 2],
    #[serde(default)]
    pub map_cal_a_pix: [f64; 2],
    #[serde(default)]
    pub map_cal_b_world: [f64; 2],
    #[serde(default)]
    pub map_cal_b_pix: [f64; 2],
    /// View zoom cap (may exceed tile native zoom — tiles upscale). 0 = preset.
    #[serde(default)]
    pub map_view_max_zoom: i32,
    /// Initial camera. 0 = use preset. Center is a full-resolution pixel (X, Y).
    #[serde(default)]
    pub map_default_zoom: i32,
    #[serde(default)]
    pub map_default_center: [f64; 2],

    // ── Panel visibility ──────────────────────────────────────────────────────
    #[serde(default = "Settings::default_tires_visible")]
    pub tires_visible: bool,

    // ── Audio alerts ──────────────────────────────────────────────────────────
    // Upshift: fires when power drops N% from rolling max at full throttle
    #[serde(default)]
    pub upshift_beep_enabled: bool,
    #[serde(default = "Settings::default_upshift_power_drop_pct")]
    pub upshift_power_drop_pct: f32,
    #[serde(default = "Settings::default_upshift_min_throttle")]
    pub upshift_min_throttle: f32,
    #[serde(default = "Settings::default_upshift_freq")]
    pub upshift_freq: f32,
    #[serde(default = "Settings::default_upshift_duration_ms")]
    pub upshift_duration_ms: f32,
    // Downshift reminder: fires when lugging (high throttle, low RPM)
    #[serde(default)]
    pub downshift_beep_enabled: bool,
    #[serde(default = "Settings::default_downshift_low_rpm_pct")]
    pub downshift_low_rpm_pct: f32,
    #[serde(default = "Settings::default_downshift_min_throttle")]
    pub downshift_min_throttle: f32,
    #[serde(default = "Settings::default_downshift_freq")]
    pub downshift_freq: f32,
    #[serde(default = "Settings::default_downshift_duration_ms")]
    pub downshift_duration_ms: f32,
    #[serde(default = "Settings::default_beep_volume")]
    pub beep_volume: f32,
}

impl Settings {
    fn default_theme() -> String { "dark".to_string() }
    fn default_map_max_zoom() -> i32 { 5 }
    fn default_map_tile_size() -> i32 { 256 }
    fn default_tires_visible() -> bool { true }
    fn default_upshift_power_drop_pct() -> f32 { 3.0 }
    fn default_upshift_min_throttle() -> f32 { 90.0 }
    fn default_upshift_freq() -> f32 { 1800.0 }
    fn default_upshift_duration_ms() -> f32 { 120.0 }
    fn default_downshift_low_rpm_pct() -> f32 { 35.0 }
    fn default_downshift_min_throttle() -> f32 { 50.0 }
    fn default_downshift_freq() -> f32 { 1200.0 }
    fn default_downshift_duration_ms() -> f32 { 100.0 }
    fn default_beep_volume() -> f32 { 0.8 }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            port: 20440,
            use_mph: true,
            tire_temp_cold: 60.0,
            tire_temp_optimal: 85.0,
            tire_temp_hot: 110.0,
            auto_record: true,
            theme: Self::default_theme(),
            map_enabled: false,
            map_override: false,
            map_tile_url: String::new(),
            map_min_zoom: 0,
            map_max_zoom: Self::default_map_max_zoom(),
            map_tile_size: Self::default_map_tile_size(),
            map_cal_a_world: [0.0, 0.0],
            map_cal_a_pix: [0.0, 0.0],
            map_cal_b_world: [0.0, 0.0],
            map_cal_b_pix: [0.0, 0.0],
            map_view_max_zoom: 0,
            map_default_zoom: 0,
            map_default_center: [0.0, 0.0],
            tires_visible: true,
            upshift_beep_enabled: false,
            upshift_power_drop_pct: Self::default_upshift_power_drop_pct(),
            upshift_min_throttle: Self::default_upshift_min_throttle(),
            upshift_freq: Self::default_upshift_freq(),
            upshift_duration_ms: Self::default_upshift_duration_ms(),
            downshift_beep_enabled: false,
            downshift_low_rpm_pct: Self::default_downshift_low_rpm_pct(),
            downshift_min_throttle: Self::default_downshift_min_throttle(),
            downshift_freq: Self::default_downshift_freq(),
            downshift_duration_ms: Self::default_downshift_duration_ms(),
            beep_volume: Self::default_beep_volume(),
        }
    }
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

fn settings_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("fh6-tel")
        .join("settings.json")
}

pub fn load() -> Settings {
    let path = settings_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(s: &Settings) -> Result<(), SettingsError> {
    let path = settings_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(&path, serde_json::to_string_pretty(s)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_port_is_20440() {
        let s = Settings::default();
        assert_eq!(s.port, 20440);
    }

    #[test]
    fn legacy_json_without_map_fields_loads_defaults() {
        // A settings.json written before the map feature existed.
        let legacy = r#"{"port":20440,"useMph":true,"tireTempCold":60.0,
            "tireTempOptimal":85.0,"tireTempHot":110.0,"autoRecord":true,"theme":"dark"}"#;
        let s: Settings = serde_json::from_str(legacy).unwrap();
        assert!(!s.map_enabled);
        assert_eq!(s.map_tile_url, "");
        assert_eq!(s.map_max_zoom, 5);
        assert_eq!(s.map_tile_size, 256);
        assert_eq!(s.map_cal_a_world, [0.0, 0.0]);
    }

    #[test]
    fn legacy_json_without_tires_visible_defaults_to_true() {
        let legacy = r#"{"port":20440,"useMph":true,"tireTempCold":60.0,
            "tireTempOptimal":85.0,"tireTempHot":110.0,"autoRecord":true,"theme":"dark"}"#;
        let s: Settings = serde_json::from_str(legacy).unwrap();
        assert!(s.tires_visible);
    }

    #[test]
    fn legacy_json_without_audio_fields_uses_defaults() {
        let legacy = r#"{"port":20440,"useMph":true,"tireTempCold":60.0,
            "tireTempOptimal":85.0,"tireTempHot":110.0,"autoRecord":true,"theme":"dark"}"#;
        let s: Settings = serde_json::from_str(legacy).unwrap();
        assert!(!s.upshift_beep_enabled);
        assert!(!s.downshift_beep_enabled);
        assert_eq!(s.upshift_power_drop_pct, 3.0);
        assert_eq!(s.upshift_min_throttle, 90.0);
        assert_eq!(s.downshift_low_rpm_pct, 35.0);
        assert_eq!(s.beep_volume, 0.8);
    }

    #[test]
    fn roundtrip_to_json() {
        let s = Settings { port: 9999, use_mph: false, ..Settings::default() };
        let json = serde_json::to_string(&s).unwrap();
        let s2: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(s2.port, 9999);
        assert!(!s2.use_mph);
    }
}
