//! Demonstrates the `EngagementPlanner` trait — creating engagement plans
//! from threat assessments and interceptor pairings.

use furia_sdk::cuas::{
    DroneClass, DroneTrack, EngagementPlan, InterceptorPairing, ThreatAssessment,
};
use furia_sdk::engagement_planner::EngagementPlanner;
use furia_sdk::module_handle::{ModuleHealth, ModuleHandle};
use furia_sdk::platform::ThreatLevel;
use uuid::Uuid;

const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;

struct DemoEngagementPlanner {
    approved_plans: Vec<String>,
}

impl DemoEngagementPlanner {
    fn new() -> Self {
        Self { approved_plans: vec![] }
    }
}

impl EngagementPlanner for DemoEngagementPlanner {
    fn init(&mut self, _handle: &ModuleHandle) {}

    fn plan(&self, threats: &[DroneTrack], _assessments: &[ThreatAssessment], pairings: &[InterceptorPairing]) -> EngagementPlan {
        let total_pk = pairings.iter().map(|p| p.pk).sum::<f64>() / pairings.len().max(1) as f64;
        EngagementPlan {
            plan_id: format!("plan-{}", threats.first().map_or("?", |t| &t.track_id)),
            generated_at_ms: 1000,
            pairings: pairings.to_vec(),
            total_pk,
            collateral_risk: 0.1,
            approved: false,
        }
    }

    fn approve(&mut self, plan_id: &str) {
        self.approved_plans.push(plan_id.into());
    }

    fn reject(&mut self, _plan_id: &str) {}

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut planner = DemoEngagementPlanner::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    planner.init(&handle);

    let track = DroneTrack {
        track_id: "THREAT-01".into(), lat: DEMO_LAT, lon: DEMO_LON, altitude_m: 800.0,
        speed_kph: 100.0, heading_deg: 270.0, drone_class: DroneClass::FixedWingMedium,
        confidence: 0.9, sensor_type: "radar".into(), first_seen_ms: 0, last_seen_ms: 5000,
    };

    let assessment = ThreatAssessment {
        track_id: track.track_id.clone(), drone_class: track.drone_class.clone(),
        threat_score: 0.8, threat_level: ThreatLevel::High, time_to_impact_s: Some(120.0),
        distance_to_asset_km: 5.0, closest_asset: "ASSET-A".into(),
        rationale: "Medium fixed-wing approaching restricted zone".into(),
    };

    let pairings = vec![InterceptorPairing {
        track_id: track.track_id.clone(), asset_id: "CIR-01".into(), pk: 0.85,
        time_to_engage_s: 30.0, recommended: true,
        rationale: "Best Pk match for fixed-wing medium".into(),
    }];

    let plan = planner.plan(&[track], &[assessment], &pairings);
    planner.approve(&plan.plan_id);

    println!("=== Engagement Plan ===");
    println!(" Plan: {}", plan.plan_id);
    println!(" Total Pk: {:.2}", plan.total_pk);
    println!(" Pairings: {}", plan.pairings.len());
    println!(" Approved: {}", plan.approved);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_track() -> DroneTrack {
        DroneTrack {
            track_id: "T1".into(), lat: 0.0, lon: 0.0, altitude_m: 500.0, speed_kph: 80.0,
            heading_deg: 0.0, drone_class: DroneClass::FixedWingSmall, confidence: 0.8,
            sensor_type: "r".into(), first_seen_ms: 0, last_seen_ms: 0,
        }
    }

    fn make_assessment(track_id: &str) -> ThreatAssessment {
        ThreatAssessment {
            track_id: track_id.into(), drone_class: DroneClass::FixedWingSmall, threat_score: 0.7,
            threat_level: ThreatLevel::High, time_to_impact_s: Some(60.0), distance_to_asset_km: 3.0,
            closest_asset: "A1".into(), rationale: "test".into(),
        }
    }

    fn make_pairing(track_id: &str) -> InterceptorPairing {
        InterceptorPairing {
            track_id: track_id.into(), asset_id: "INT-1".into(), pk: 0.8, time_to_engage_s: 20.0,
            recommended: true, rationale: "best match".into(),
        }
    }

    #[test]
    fn test_plan_creates_engagement_plan() {
        let p = DemoEngagementPlanner::new();
        let plan = p.plan(&[make_track()], &[make_assessment("T1")], &[make_pairing("T1")]);
        assert_eq!(plan.plan_id, "plan-T1");
        assert!(!plan.approved);
    }

    #[test]
    fn test_approve_records_plan() {
        let mut p = DemoEngagementPlanner::new();
        p.approve("plan-T1");
        assert_eq!(p.approved_plans, vec!["plan-T1"]);
    }

    #[test]
    fn test_total_pk_average() {
        let p = DemoEngagementPlanner::new();
        let plan = p.plan(&[make_track()], &[], &[make_pairing("T1"), make_pairing("T1")]);
        assert!((plan.total_pk - 0.8).abs() < 0.01);
    }
}
