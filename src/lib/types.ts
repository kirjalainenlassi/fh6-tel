export interface TelemetryPacket {
  isRaceOn: boolean;
  timestampMs: number;
  engineMaxRpm: number;
  engineIdleRpm: number;
  currentEngineRpm: number;
  accelX: number;
  accelY: number;
  accelZ: number;
  velX: number;
  velY: number;
  velZ: number;
  positionX: number;
  positionY: number;
  positionZ: number;
  tireSlipRatioFl: number;
  tireSlipRatioFr: number;
  tireSlipRatioRl: number;
  tireSlipRatioRr: number;
  tireSlipAngleFl: number;
  tireSlipAngleFr: number;
  tireSlipAngleRl: number;
  tireSlipAngleRr: number;
  carOrdinal: number;
  carClass: number;
  carPi: number;
  drivetrainType: number;
  speedMs: number;
  power: number;
  torque: number;
  tireTempFl: number;
  tireTempFr: number;
  tireTempRl: number;
  tireTempRr: number;
  boost: number;
  fuel: number;
  distanceTraveled: number;
  bestLap: number;
  lastLap: number;
  currentLap: number;
  currentRaceTime: number;
  lapNumber: number;
  racePosition: number;
  throttle: number;
  brake: number;
  clutch: number;
  handbrake: number;
  gear: number;
  steer: number;
  yaw: number;
  pitch: number;
  roll: number;
  suspensionFl: number;
  suspensionFr: number;
  suspensionRl: number;
  suspensionRr: number;
  tireWearFl: number | null;
  tireWearFr: number | null;
  tireWearRl: number | null;
  tireWearRr: number | null;
}

export interface SessionRow {
  id: number;
  startedAt: number;
  endedAt: number | null;
  carOrdinal: number;
  carClass: number;
  carPi: number;
  bestLap: number | null;
  packetCount: number;
  name: string | null;
  bookmarked: boolean;
}

export interface SessionLap {
  lapNumber: number;
  lapTime: number;
}

export interface AppSettings {
  port: number;
  useMph: boolean;
  tireTempCold: number;
  tireTempOptimal: number;
  tireTempHot: number;
  autoRecord: boolean;
  theme: 'dark' | 'cobalt2' | 'purple';
  mapEnabled: boolean;
  mapOverride: boolean;
  mapTileUrl: string;
  mapMinZoom: number;
  mapMaxZoom: number;
  mapTileSize: number;
  mapCalAWorld: [number, number];
  mapCalAPix: [number, number];
  mapCalBWorld: [number, number];
  mapCalBPix: [number, number];
  mapViewMaxZoom: number;
  mapDefaultZoom: number;
  mapDefaultCenter: [number, number];
  tiresVisible: boolean;

  // ── Audio alerts ──────────────────────────────────────────────────────────
  // Upshift: beeps when engine power drops N% from rolling max at full throttle
  upshiftBeepEnabled: boolean;
  upshiftPowerDropPct: number;  // % drop from rolling max power, default 3
  upshiftMinThrottle: number;   // throttle % required to track power, default 90
  upshiftFreq: number;          // Hz, default 1800
  upshiftDurationMs: number;    // ms, default 120
  // Downshift reminder: beeps when lugging (high throttle, low RPM for current gear)
  downshiftBeepEnabled: boolean;
  downshiftLowRpmPct: number;   // RPM% below which "lugging" fires, default 35
  downshiftMinThrottle: number; // min throttle % to detect lugging, default 50
  downshiftFreq: number;        // Hz, default 1200
  downshiftDurationMs: number;  // ms, default 100
  beepVolume: number;           // 0–1, default 0.8
}

export type DrivetrainLabel = 'FWD' | 'RWD' | 'AWD';
export const DRIVETRAIN_LABELS: DrivetrainLabel[] = ['FWD', 'RWD', 'AWD'];

export type CarClassLabel = 'D' | 'C' | 'B' | 'A' | 'S1' | 'S2' | 'X';
export const CAR_CLASS_LABELS: CarClassLabel[] = ['D', 'C', 'B', 'A', 'S1', 'S2', 'X'];
