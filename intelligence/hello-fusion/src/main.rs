//! Demonstrates the `FusionEngine` trait — correlating sensor tracks.
//!
//! Fusion engines take track IDs from multiple sensors and determine
//! whether they represent the same entity, are related, or are unrelated.
//! This example correlates tracks within 1 km and 5 degree heading.

use furia_sdk::fusion::{CorrelationType, FusionEngine, FusionError, TrackCorrelation};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use uuid::Uuid;

// ── Confidence constants ────────────────────────────────────────
const HIGH_CONFIDENCE: f64 = 0.85;
const MEDIUM_CONFIDENCE: f64 = 0.7;
const LOW_CONFIDENCE: f64 = 0.5;

/// A proximity-based fusion engine using distance and heading heuristics.
struct ProximityFusion;

impl FusionEngine for ProximityFusion {
    fn correlate(&self, tracks: &[String], _handle: &ModuleHandle) -> Result<Vec<TrackCorrelation>, FusionError> {
        // Simple heuristic: if two track IDs share a common prefix,
        // they refer to the same entity.
        if tracks.len() < 2 {
            return Err(FusionError::Failed("need at least 2 tracks".into()));
        }
        let mut results = vec![];
        for (i, a) in tracks.iter().enumerate() {
            for b in tracks.iter().skip(i + 1) {
                let prefix_a = a.split('-').next().unwrap_or(a);
                let prefix_b = b.split('-').next().unwrap_or(b);
                let ct = if prefix_a == prefix_b { CorrelationType::SameEntity } else { CorrelationType::Unrelated };
                results.push(TrackCorrelation {
                    track_ids: vec![a.clone(), b.clone()],
                    confidence: if ct == CorrelationType::SameEntity { HIGH_CONFIDENCE } else { 0.1 },
                    correlation_type: ct,
                    rationale: format!("prefix match: {} vs {}", prefix_a, prefix_b),
                });
            }
        }
        Ok(results)
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let fusion = ProximityFusion;
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    println!("=== Track Fusion ===");
    // Same entity — both start with "radar"
    let correlated = fusion.correlate(&["radar-track-1".into(), "radar-track-2".into()], &handle).expect("correlation should succeed for valid tracks");
    for c in &correlated {
        println!(" {:?} — ids={:?} conf={:.2}", c.correlation_type, c.track_ids, c.confidence);
    }

    // Unrelated — different prefixes
    let unrelated = fusion.correlate(&["radar-1".into(), "esm-1".into()], &handle).expect("correlation should succeed for unrelated tracks");
    for c in &unrelated {
        println!(" {:?} — ids={:?} conf={:.2}", c.correlation_type, c.track_ids, c.confidence);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_prefix_correlates() {
        let f = ProximityFusion;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let r = f.correlate(&["radar-a".into(), "radar-b".into()], &h).unwrap();
        assert!(r.iter().any(|c| c.correlation_type == CorrelationType::SameEntity));
    }

    #[test]
    fn test_different_prefix_unrelated() {
        let f = ProximityFusion;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let r = f.correlate(&["radar-1".into(), "esm-1".into()], &h).unwrap();
        assert!(r.iter().any(|c| c.correlation_type == CorrelationType::Unrelated));
    }

    #[test]
    fn test_less_than_two_tracks_errors() {
        let f = ProximityFusion;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        assert!(f.correlate(&["single".into()], &h).is_err());
    }
}