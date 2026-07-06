//! Demonstrates the `ThreatScorer` trait — classifying drone tracks and
//! assigning threat scores based on proximity to sensitive zones.

use furia_sdk::cuas::{DroneClass, DroneTrack, ThreatAssessment};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::platform::ThreatLevel;
use furia_sdk::simulation::Scenario;
use furia_sdk::ThreatScorer;
use uuid::Uuid;

const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;

struct DemoThreatScorer {
    scenario: Option<Scenario>,
}

impl DemoThreatScorer {
    fn new() -> Self {
        Self { scenario: None }
    }
}

impl furia_sdk::threat_scorer::ThreatScorer for DemoThreatScorer {
    fn init(&mut self, scenario: &Scenario, _handle: &ModuleHandle) {
        self.scenario = Some(scenario.clone());
    }

    fn score(&self, track: &DroneTrack, sensitive_zones: &[String]) -> ThreatAssessment {
        let mut score: f64 = 0.0;
        let mut rationale: Vec<String> = vec![];

        match track.drone_class {
            DroneClass::FixedWingLarge | DroneClass::LoiteringMunition => {
                score += 0.3;
                rationale.push("high-capability drone class".into());
            }
            DroneClass::QuadcopterMedium | DroneClass::FixedWingMedium => {
                score += 0.2;
                rationale.push("medium-capability drone class".into());
            }
            _ => {
                score += 0.1;
                rationale.push("low-capability drone class".into());
            }
        }

        if track.speed_kph > 150.0 {
            score += 0.2;
            rationale.push("high speed".into());
        }

        for zone in sensitive_zones {
            score += 0.1;
            rationale.push(format!("near {}", zone));
        }

        let threat_level = if score >= 0.7 { ThreatLevel::Critical }
            else if score >= 0.5 { ThreatLevel::High }
            else if score >= 0.3 { ThreatLevel::Medium }
            else { ThreatLevel::Low };

        ThreatAssessment {
            track_id: track.track_id.clone(),
            drone_class: track.drone_class.clone(),
            threat_score: (score).min(1.0),
            threat_level,
            time_to_impact_s: None,
            distance_to_asset_km: 0.0,
            closest_asset: sensitive_zones.first().cloned().unwrap_or_default(),
            rationale: rationale.join("; "),
        }
    }

    fn batch_score(&self, tracks: &[DroneTrack], sensitive_zones: &[String]) -> Vec<ThreatAssessment> {
        tracks.iter().map(|t| self.score(t, sensitive_zones)).collect()
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut scorer = DemoThreatScorer::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    let scenario = Scenario {
        id: "scenario-001".into(), name: "demo".into(), duration_secs: 3600,
        order_of_battle: serde_json::json!({}), timeline: vec![],
        environment: serde_json::json!({}),
    };
    scorer.init(&scenario, &handle);

    let track = DroneTrack {
        track_id: "DRONE-H".into(), lat: DEMO_LAT, lon: DEMO_LON, altitude_m: 800.0,
        speed_kph: 200.0, heading_deg: 270.0, drone_class: DroneClass::FixedWingMedium,
        confidence: 0.9, sensor_type: "radar".into(), first_seen_ms: 0, last_seen_ms: 5000,
    };

    let zones = vec!["airbase-main".into(), "ammo-depot".into()];
    let assessment = scorer.score(&track, &zones);

    println!("=== Threat Assessment ===");
    println!(" Track: {}", assessment.track_id);
    println!(" Class: {:?}", assessment.drone_class);
    println!(" Score: {:.2}", assessment.threat_score);
    println!(" Level: {:?}", assessment.threat_level);
    println!(" Rationale: {}", assessment.rationale);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_track(dc: DroneClass, speed: f64) -> DroneTrack {
        DroneTrack {
            track_id: "T1".into(), lat: 0.0, lon: 0.0, altitude_m: 500.0, speed_kph: speed,
            heading_deg: 0.0, drone_class: dc, confidence: 0.8, sensor_type: "r".into(),
            first_seen_ms: 0, last_seen_ms: 0,
        }
    }

    #[test]
    fn test_loitering_munition_scores_high() {
        let s = DemoThreatScorer::new();
        let a = s.score(&make_track(DroneClass::LoiteringMunition, 200.0), &["zone-a".into(), "zone-b".into()]);
        assert!(a.threat_score >= 0.7);
        assert_eq!(a.threat_level, ThreatLevel::Critical);
    }

    #[test]
    fn test_low_speed_quadcopter_scores_low() {
        let s = DemoThreatScorer::new();
        let a = s.score(&make_track(DroneClass::QuadcopterSmall, 30.0), &[]);
        assert!(a.threat_score < 0.3);
    }

    #[test]
    fn test_batch_score_returns_all_assessments() {
        let s = DemoThreatScorer::new();
        let tracks = vec![make_track(DroneClass::QuadcopterSmall, 30.0), make_track(DroneClass::FixedWingLarge, 300.0)];
        let assessments = s.batch_score(&tracks, &[]);
        assert_eq!(assessments.len(), 2);
        assert!(assessments[1].threat_score > assessments[0].threat_score);
    }
}
