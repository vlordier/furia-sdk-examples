//! Demonstrates the `EWSimProvider` trait — jamming a radio link.
//!
//! Electronic Warfare simulation configures jammers against emitter
//! profiles, advances them over time, and performs ESM sweeps to detect
//! active emitters in the environment.

use std::collections::HashMap;
use std::time::Duration;

use furia_sdk::ew_sim::{DeconflictionZone, EmitterProfile, EsmDetection, EWSimProvider, EwSimSnapshot, JammerConfig, JammingTechnique, SpectrumAllocation, SpectrumState};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::{Scenario, SimEvent};
use uuid::Uuid;

/// An EW simulator that jams a single emitter and sweeps for others.
struct JammerSim {
    jammer: Option<JammerConfig>,
    target: Option<EmitterProfile>,
    t_ms: u64,
}

impl JammerSim {
    fn new() -> Self { Self { jammer: None, target: None, t_ms: 0 } }
}

impl EWSimProvider for JammerSim {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) {}

    fn configure_jammer(&mut self, emitter: EmitterProfile, jammer: JammerConfig) {
        self.target = Some(emitter);
        self.jammer = Some(jammer);
    }

    fn tick_jammers(&mut self, dt: Duration) -> Vec<SimEvent> {
        self.t_ms += dt.as_millis() as u64;
        vec![SimEvent {
            event_type: "jamming".into(), source: "jammer-001".into(), target: None,
            params: serde_json::json!({"elapsed_ms": self.t_ms}),
            timestamp_ms: self.t_ms,
        }]
    }

    fn esm_sweep(&self) -> Vec<EsmDetection> {
        vec![EsmDetection {
            emitter_id: "emitter-radar-a".into(), signal_strength_dbm: -45.0,
            confidence: 0.8, bearing_deg: 270.0, first_detected_ms: 0,
        }]
    }

    fn spectrum_state(&self) -> SpectrumState {
        SpectrumState {
            allocations: vec![SpectrumAllocation {
                allocation_id: "alloc-1".into(), owner: "jammer-001".into(),
                freq_min_hz: 2.4e9, freq_max_hz: 2.5e9, priority: 1,
            }],
            deconfliction_zones: vec![DeconflictionZone {
                zone_id: "dz-1".into(), polygon: vec![(48.0, 2.0), (49.0, 3.0)],
                restricted_freqs: vec![(2.4e9, 2.5e9)], effective_from_ms: 0, effective_to_ms: 3600000,
            }],
        }
    }

    fn snapshot(&self) -> EwSimSnapshot {
        EwSimSnapshot {
            active_jammers: self.jammer.clone().into_iter().collect(),
            detected_emitters: self.esm_sweep(),
            spectrum_state: self.spectrum_state(),
            comms_degradation: HashMap::from([("link-001".into(), 0.85)]),
        }
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut ew = JammerSim::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    let scenario = Scenario {
        id: "ew-test".into(), name: "EW Demo".into(), duration_secs: 300,
        order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({}),
    };
    ew.init(&scenario, &handle);

    println!("=== Electronic Warfare Simulation ===");
    ew.configure_jammer(
        EmitterProfile { emitter_id: "radar-a".into(), emitter_type: "search".into(), frequency_hz: 3.0e9, power_dbm: 50.0, beam_width_deg: 30.0 },
        JammerConfig { jammer_id: "jammer-001".into(), technique: JammingTechnique::Barrage, center_freq_hz: 3.0e9, bandwidth_hz: 100.0e6, erp_dbm: 60.0 },
    );

    let _events = ew.tick_jammers(Duration::from_secs(10));
    println!(" ESM sweep: {} detections", ew.esm_sweep().len());
    println!(" Snapshot jammers: {}", ew.snapshot().active_jammers.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configure_jammer() {
        let mut ew = JammerSim::new();
        let emitter = EmitterProfile { emitter_id: "e1".into(), emitter_type: "t".into(), frequency_hz: 1e9, power_dbm: 30.0, beam_width_deg: 10.0 };
        let jammer = JammerConfig { jammer_id: "j1".into(), technique: JammingTechnique::Spot, center_freq_hz: 1e9, bandwidth_hz: 1e6, erp_dbm: 50.0 };
        ew.configure_jammer(emitter, jammer);
        assert!(ew.jammer.is_some());
    }

    #[test]
    fn test_esm_sweep_returns_detections() {
        let ew = JammerSim::new();
        assert!(!ew.esm_sweep().is_empty());
    }

    #[test]
    fn test_snapshot_contains_comms_degradation() {
        let ew = JammerSim::new();
        assert!(ew.snapshot().comms_degradation.contains_key("link-001"));
    }
}