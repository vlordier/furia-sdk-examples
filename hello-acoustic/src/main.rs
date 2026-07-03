//! Demonstrates the `AcousticProvider` trait — passive sonar detection.
//!
//! Underwater acoustic simulation models sound propagation, sonar
//! detection ranges (active/passive), and submarine contact tracking
//! for anti-submarine warfare (USW) operations.

use std::time::Duration;

use furia_sdk::acoustic::{AcousticPropagation, AcousticProvider, AcousticSnapshot, SonarConfig, SonarDetection, SonarType};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::{Scenario, SimEvent};
use uuid::Uuid;

/// A passive sonar simulator that detects a submerged contact.
struct PassiveSonar {
    configs: Vec<SonarConfig>,
    detections: Vec<SonarDetection>,
    t_ms: u64,
}

impl PassiveSonar {
    fn new() -> Self { Self { configs: vec![], detections: vec![], t_ms: 0 } }
}

impl AcousticProvider for PassiveSonar {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) {}

    fn configure_sonar(&mut self, config: SonarConfig) {
        self.configs.push(config);
    }

    fn tick_acoustics(&mut self, dt: Duration) -> Vec<SimEvent> {
        self.t_ms += dt.as_millis() as u64;
        self.detections = vec![SonarDetection {
            contact_id: "sub-001".into(),
            estimated_position: (48.80, 2.30, -150.0),
            confidence: 0.72,
            classification: "submarine".into(),
            signal_excess_db: 8.0,
        }];
        vec![]
    }

    fn propagation_conditions(&self, lat: f64, lon: f64, depth_m: f64) -> AcousticPropagation {
        AcousticPropagation {
            lat, lon, depth_m,
            transmission_loss_db: 15.0 + depth_m * 0.02,
            convergence_zone_km: 35.0,
            shadow_zones: vec![],
        }
    }

    fn sonar_detections(&self) -> Vec<SonarDetection> { self.detections.clone() }

    fn snapshot(&self) -> AcousticSnapshot {
        AcousticSnapshot {
            propagation_conditions: vec![],
            active_detections: self.detections.clone(),
            ambient_noise_db: 55.0,
            sonar_configs: self.configs.clone(),
        }
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut sonar = PassiveSonar::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    let scenario = Scenario { id: "usw".into(), name: "USW Demo".into(), duration_secs: 7200, order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({}) };
    sonar.init(&scenario, &handle);

    println!("=== Acoustic / Sonar ===");
    sonar.configure_sonar(SonarConfig {
        platform_id: "p-3c".into(), type_: SonarType::Passive,
        source_level_db: 0.0, frequency_hz: 1000.0, beam_width_deg: 60.0,
    });
    sonar.tick_acoustics(Duration::from_secs(30));

    let prop = sonar.propagation_conditions(48.85, 2.35, 100.0);
    println!(" Propagation loss at 100m: {:.1} dB", prop.transmission_loss_db);
    for d in sonar.sonar_detections() {
        println!(" Detection: {} ({}) conf={:.2}", d.contact_id, d.classification, d.confidence);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configure_sonar_adds_config() {
        let mut s = PassiveSonar::new();
        let cfg = SonarConfig { platform_id: "p1".into(), type_: SonarType::Active, source_level_db: 200.0, frequency_hz: 5000.0, beam_width_deg: 30.0 };
        s.configure_sonar(cfg);
        assert_eq!(s.configs.len(), 1);
    }

    #[test]
    fn test_tick_generates_detections() {
        let mut s = PassiveSonar::new();
        s.tick_acoustics(Duration::from_secs(10));
        assert!(!s.sonar_detections().is_empty());
    }

    #[test]
    fn test_propagation_loss_increases_with_depth() {
        let s = PassiveSonar::new();
        let shallow = s.propagation_conditions(0.0, 0.0, 10.0);
        let deep = s.propagation_conditions(0.0, 0.0, 500.0);
        assert!(deep.transmission_loss_db > shallow.transmission_loss_db);
    }
}