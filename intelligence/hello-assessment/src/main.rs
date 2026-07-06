//! Demonstrates the `AssessmentEngine` trait — battle damage assessment.
//!
//! An assessment engine analyses after-action `SimEvent`s to determine
//! target status (destroyed, damaged, missed), estimate confidence, and
//! flag possible collateral harm.

use furia_sdk::assessor::{AssessmentEngine, AssessmentError, BattleDamageAssessment};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::SimEvent;
use uuid::Uuid;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;
const DEMO_DURATION_SECS: u64 = 3600;

/// A simple BDA engine that checks for impact proximity indicators.
struct StrikeAssessor;

impl AssessmentEngine for StrikeAssessor {
    fn assess(&self, mission_id: &str, events: &[SimEvent], _handle: &ModuleHandle) -> Result<BattleDamageAssessment, AssessmentError> {
        let mut hit_detected = false;
        let mut collateral = false;

        for ev in events {
            if ev.event_type == "impact" && ev.target.as_deref() == Some(mission_id) {
                hit_detected = true;
            }
            if ev.event_type == "collateral" {
                collateral = true;
            }
        }

        Ok(BattleDamageAssessment {
            mission_id: mission_id.into(),
            target_status: if hit_detected { "destroyed".into() } else { "missed".into() },
            confidence: if hit_detected { 0.92 } else { 0.2 },
            collateral_reported: Some(collateral),
            remarks: if hit_detected { "Direct hit confirmed by impact sensor".into() } else { "No impact event observed".into() },
        })
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let assessor = StrikeAssessor;
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    let events = vec![
        SimEvent {
            event_type: "impact".into(), source: "weapon".into(),
            target: Some("strike-001".into()),
            params: serde_json::json!({"offset_m": 2.5}),
            timestamp_ms: 1000,
        },
        SimEvent {
            event_type: "splash".into(), source: "sensor".into(),
            target: Some("strike-001".into()),
            params: serde_json::json!({"visible": true}),
            timestamp_ms: 2000,
        },
    ];

    println!("=== Battle Damage Assessment ===");
    let bda = assessor.assess("strike-001", &events, &handle).expect("BDA assessment should succeed with valid events");
    println!(" Status: {} (confidence {:.2})", bda.target_status, bda.confidence);
    println!(" Collateral reported: {:?}", bda.collateral_reported);
    println!(" Remarks: {}", bda.remarks);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_detected() {
        let a = StrikeAssessor;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let ev = vec![SimEvent { event_type: "impact".into(), source: "w".into(), target: Some("m1".into()), params: serde_json::json!({}), timestamp_ms: 0 }];
        let bda = a.assess("m1", &ev, &h).unwrap();
        assert_eq!(bda.target_status, "destroyed");
    }

    #[test]
    fn test_miss_when_no_impact() {
        let a = StrikeAssessor;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let bda = a.assess("m1", &[], &h).unwrap();
        assert_eq!(bda.target_status, "missed");
    }

    #[test]
    fn test_collateral_flag() {
        let a = StrikeAssessor;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let ev = vec![SimEvent { event_type: "collateral".into(), source: "s".into(), target: None, params: serde_json::json!({}), timestamp_ms: 0 }];
        let bda = a.assess("m1", &ev, &h).unwrap();
        assert_eq!(bda.collateral_reported, Some(true));
    }
}