//! Demonstrates the `DecompositionStrategy` trait — splitting a mission
//! into Find, Fix, Track, Target, Engage, Assess (F2T2EA) phases.
//!
//! Decomposition converts a commander's high-level intent into a set of
//! actionable sub-missions, each with assigned assets and waypoints.
//! It also scores Courses of Action for comparison.

use furia_sdk::decomposition::{CoAScore, DecompositionError, DecompositionStrategy, SubMission};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use uuid::Uuid;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;
const DEMO_DURATION_SECS: u64 = 3600;

// ── Confidence constants ────────────────────────────────────────
const HIGH_CONFIDENCE: f64 = 0.85;
const MEDIUM_CONFIDENCE: f64 = 0.7;
const LOW_CONFIDENCE: f64 = 0.5;

/// Splits a mission into standard F2T2EA sub-missions.
struct F2T2EAStrategy;

impl DecompositionStrategy for F2T2EAStrategy {
    fn decompose(&self, intent: &str, _handle: &ModuleHandle) -> Result<Vec<SubMission>, DecompositionError> {
        // Parse the intent roughly — for demo we use a fixed pattern
        let target = intent.split(" at ").next().unwrap_or("unknown");
        Ok(vec![
            SubMission {
                id: "sub-find".into(), objective: format!("Find {}", target),
                asset_ids: vec!["uav-001".into()],
                waypoints: vec![(DEMO_LAT, DEMO_LON, Some(500.0))],
                constraints: serde_json::json!({"max_alt_m": 1000}),
            },
            SubMission {
                id: "sub-fix".into(), objective: format!("Fix {} position", target),
                asset_ids: vec!["uav-001".into()],
                waypoints: vec![(DEMO_LAT, DEMO_LON, Some(300.0))],
                constraints: serde_json::json!({"loiter_s": 300}),
            },
            SubMission {
                id: "sub-engage".into(), objective: format!("Engage {}", target),
                asset_ids: vec!["strike-001".into()],
                waypoints: vec![(DEMO_LAT, DEMO_LON, Some(0.0))],
                constraints: serde_json::json!({"weapon": "pgm"}),
            },
        ])
    }

    fn score_coa(&self, _mission_id: &str, _handle: &ModuleHandle) -> Result<CoAScore, DecompositionError> {
        Ok(CoAScore {
            numeric_score: 0.78,
            factors: vec![
                ("risk".into(), 0.65),
                ("speed".into(), HIGH_CONFIDENCE),
                ("economy_of_force".into(), 0.72),
            ],
        })
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let strategy = F2T2EAStrategy;
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    println!("=== Decomposition: 'destroy T72 at {:.2}N {:.2}E' ===", DEMO_LAT, DEMO_LON);
    let missions = strategy.decompose(&format!("T72 at {:.2}N {:.2}E", DEMO_LAT, DEMO_LON), &handle).expect("decompose should succeed for valid intent");
    for m in &missions {
        println!(" {} — {} (assets: {:?})", m.id, m.objective, m.asset_ids);
    }

    let score = strategy.score_coa("mission-001", &handle).expect("COA scoring should succeed");
    println!("\n=== COA Score: {:.2} ===", score.numeric_score);
    for (factor, val) in &score.factors {
        println!(" {}: {:.2}", factor, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_produces_sub_missions() {
        let s = F2T2EAStrategy;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let ms = s.decompose(&format!("destroy tank at {:.2}N {:.2}E", DEMO_LAT, DEMO_LON), &h).unwrap();
        assert_eq!(ms.len(), 3);
    }

    #[test]
    fn test_decompose_contains_engage() {
        let s = F2T2EAStrategy;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let ms = s.decompose("T72", &h).unwrap();
        assert!(ms.iter().any(|m| m.id == "sub-engage"));
    }

    #[test]
    fn test_score_is_in_range() {
        let s = F2T2EAStrategy;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let score = s.score_coa("test", &h).unwrap();
        assert!(score.numeric_score > 0.0 && score.numeric_score <= 1.0);
    }
}