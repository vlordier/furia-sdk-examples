//! Demonstrates the `KillChainOrchestrator` trait — automating the
//! Detect-Deliberate-Identify-Engage-Assess (D-DIL) chain for drone threats.

use furia_sdk::cuas::{DroneClass, DroneTrack, KillAuthority, KillChainState, KillChainStep, ThreatAssessment};
use furia_sdk::kill_chain::KillChainOrchestrator;
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::platform::ThreatLevel;
use furia_sdk::simulation::Scenario;
use std::collections::HashMap;
use uuid::Uuid;

const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;

struct DemoKillChain {
    states: HashMap<String, KillChainState>,
    scenario: Option<Scenario>,
}

impl DemoKillChain {
    fn new() -> Self {
        Self { states: HashMap::new(), scenario: None }
    }
}

impl KillChainOrchestrator for DemoKillChain {
    fn init(&mut self, scenario: &Scenario, _handle: &ModuleHandle) {
        self.scenario = Some(scenario.clone());
    }

    fn process(&self, track: &DroneTrack, assessment: &ThreatAssessment) -> KillChainState {
        let step = if assessment.threat_score >= 0.8 {
            KillChainStep::Lethal(furia_sdk::cuas::EngagementPlan {
                plan_id: format!("ep-{}", track.track_id),
                generated_at_ms: 1000, pairings: vec![], total_pk: 0.0,
                collateral_risk: 0.0, approved: false,
            })
        } else if assessment.threat_score >= 0.5 {
            KillChainStep::Identify
        } else {
            KillChainStep::Detect
        };
        KillChainState {
            track_id: track.track_id.clone(),
            current_step: step,
            started_at_ms: 0, last_updated_ms: 1000,
            authority: KillAuthority::SemiAutomatic,
        }
    }

    fn authorize(&mut self, track_id: &str, authority: KillAuthority) {
        if let Some(state) = self.states.get_mut(track_id) {
            state.authority = authority;
        }
    }

    fn state(&self, track_id: &str) -> Option<KillChainState> {
        self.states.get(track_id).cloned()
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut kc = DemoKillChain::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    let scenario = Scenario {
        id: "scenario-001".into(), name: "demo".into(), duration_secs: 3600,
        order_of_battle: serde_json::json!({}), timeline: vec![],
        environment: serde_json::json!({}),
    };
    kc.init(&scenario, &handle);

    let track = DroneTrack {
        track_id: "DRONE-H".into(), lat: DEMO_LAT, lon: DEMO_LON, altitude_m: 600.0,
        speed_kph: 120.0, heading_deg: 180.0, drone_class: DroneClass::FixedWingMedium,
        confidence: 0.92, sensor_type: "radar".into(), first_seen_ms: 0, last_seen_ms: 3000,
    };

    let assessment = ThreatAssessment {
        track_id: track.track_id.clone(), drone_class: track.drone_class.clone(),
        threat_score: 0.85, threat_level: ThreatLevel::Critical, time_to_impact_s: Some(45.0),
        distance_to_asset_km: 2.0, closest_asset: "BASE-A".into(),
        rationale: "High-speed fixed-wing on intercept course".into(),
    };

    let state = kc.process(&track, &assessment);
    println!("=== Kill Chain ===");
    println!(" Track: {}", state.track_id);
    println!(" Step: {:?}", state.current_step);
    println!(" Authority: {:?}", state.authority);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_track() -> DroneTrack {
        DroneTrack {
            track_id: "T1".into(), lat: 0.0, lon: 0.0, altitude_m: 500.0, speed_kph: 100.0,
            heading_deg: 0.0, drone_class: DroneClass::FixedWingMedium, confidence: 0.9,
            sensor_type: "r".into(), first_seen_ms: 0, last_seen_ms: 0,
        }
    }

    fn make_assessment(score: f64) -> ThreatAssessment {
        ThreatAssessment {
            track_id: "T1".into(), drone_class: DroneClass::FixedWingMedium, threat_score: score,
            threat_level: ThreatLevel::High, time_to_impact_s: Some(60.0), distance_to_asset_km: 3.0,
            closest_asset: "A1".into(), rationale: "test".into(),
        }
    }

    fn make_scenario() -> Scenario {
        Scenario {
            id: "s1".into(), name: "test".into(), duration_secs: 3600,
            order_of_battle: serde_json::json!({}), timeline: vec![],
            environment: serde_json::json!({}),
        }
    }

    #[test]
    fn test_process_high_threat_returns_lethal_step() {
        let kc = DemoKillChain::new();
        let state = kc.process(&make_track(), &make_assessment(0.85));
        assert!(matches!(state.current_step, KillChainStep::Lethal(_)));
    }

    #[test]
    fn test_process_medium_threat_returns_identify() {
        let kc = DemoKillChain::new();
        let state = kc.process(&make_track(), &make_assessment(0.6));
        assert_eq!(state.current_step, KillChainStep::Identify);
    }

    #[test]
    fn test_authorize_updates_authority() {
        let mut kc = DemoKillChain::new();
        let h = ModuleHandle::new_test(Uuid::new_v4());
        kc.init(&make_scenario(), &h);
        let state = kc.process(&make_track(), &make_assessment(0.5));
        // Manually insert since process doesn't store
        kc.states.insert("T1".into(), state);
        kc.authorize("T1", KillAuthority::AutomaticLethal);
        assert_eq!(kc.state("T1").unwrap().authority, KillAuthority::AutomaticLethal);
    }
}
