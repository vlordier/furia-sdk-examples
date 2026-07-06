//! Demonstrates the `SensorAdapter` trait — ingesting radar track data.
//!
//! A sensor adapter parses raw payload bytes into structured `TrackIngest`
//! records and can classify tracks by affiliation, type, and threat level.

use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::sensor::{AdapterError, SensorAdapter, TrackClassification, TrackIngest};
use uuid::Uuid;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;
const DEMO_DURATION_SECS: u64 = 3600;

// ── Confidence constants ────────────────────────────────────────
const HIGH_CONFIDENCE: f64 = 0.85;
const MEDIUM_CONFIDENCE: f64 = 0.7;
const LOW_CONFIDENCE: f64 = 0.5;

/// A radar sensor adapter that parses CSV payloads and classifies tracks.
struct RadarAdapter {
    classification_rules: Vec<(&'static str, &'static str)>,
}

impl RadarAdapter {
    fn new() -> Self {
        Self { classification_rules: vec![("tank", "hostile"), ("truck", "unknown")] }
    }
}

impl SensorAdapter for RadarAdapter {
    fn ingest(&self, payload: &[u8], _handle: &ModuleHandle) -> Result<Vec<TrackIngest>, AdapterError> {
        let text = std::str::from_utf8(payload).map_err(|e| AdapterError::IngestFailed(e.to_string()))?;
        let mut tracks = vec![];
        for line in text.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 4 {
                return Err(AdapterError::IngestFailed("expected CSV: id,lat,lon,alt".into()));
            }
            let parse_f64 = |s: &str, msg: &str| -> Result<f64, AdapterError> {
                s.parse().map_err(|_| AdapterError::IngestFailed(msg.into()))
            };
            tracks.push(TrackIngest {
                track_id: parts[0].into(),
                latitude: parse_f64(parts[1], "bad latitude")?,
                longitude: parse_f64(parts[2], "bad longitude")?,
                altitude: Some(parts[3].parse().unwrap_or(0.0)),
                velocity: None, heading: None, classification: None,
                confidence: 1.0, sensor_type: "radar".into(),
            });
        }
        Ok(tracks)
    }

    fn classify(&self, track: &TrackIngest, _handle: &ModuleHandle) -> Result<TrackClassification, AdapterError> {
        for (pattern, classification) in &self.classification_rules {
            if track.track_id.contains(pattern) {
                return Ok(TrackClassification {
                    track_id: track.track_id.clone(),
                    classification: classification.to_string(),
                    sub_classification: None, confidence: HIGH_CONFIDENCE as f32,
                });
            }
        }
        Ok(TrackClassification {
            track_id: track.track_id.clone(),
            classification: "friendly".into(), sub_classification: None, confidence: MEDIUM_CONFIDENCE as f32,
        })
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let radar = RadarAdapter::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    let csv = format!("tank-001,{:.2},{:.2},150\ntruck-001,48.86,2.36,0\n", DEMO_LAT, DEMO_LON);

    println!("=== Radar Ingest ===");
    let tracks = radar.ingest(csv.as_bytes(), &handle).expect("radar ingest should parse valid CSV");
    for t in &tracks {
        let cls = radar.classify(t, &handle).expect("classification should succeed for valid track");
        println!(" {} @ ({:.4}, {:.4}) → {}", t.track_id, t.latitude, t.longitude, cls.classification);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingest_parses_csv() {
        let r = RadarAdapter::new();
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let tracks = r.ingest(b"a,1.0,2.0,100\nb,3.0,4.0,200", &h).unwrap();
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].track_id, "a");
    }

    #[test]
    fn test_classify_tank_as_hostile() {
        let r = RadarAdapter::new();
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let t = TrackIngest { track_id: "tank-001".into(), latitude: 0.0, longitude: 0.0, altitude: None, velocity: None, heading: None, classification: None, confidence: 1.0, sensor_type: "t".into() };
        let c = r.classify(&t, &h).unwrap();
        assert_eq!(c.classification, "hostile");
    }

    #[test]
    fn test_invalid_csv_returns_error() {
        let r = RadarAdapter::new();
        let h = ModuleHandle::new_test(Uuid::new_v4());
        assert!(r.ingest(b"bad", &h).is_err());
    }
}