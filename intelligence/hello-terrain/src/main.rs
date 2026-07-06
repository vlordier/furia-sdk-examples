//! Demonstrates the `TerrainAnalyst` trait — slope-based mobility classification.
//!
//! Terrain analysis classifies ground by military mobility (GO / RESTRICTED /
//! NO-GO per NATO standards), assesses route trafficability, bridges, and
//! line-of-sight masking.

use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::Scenario;
use furia_sdk::terrain::{
    BridgeClassification, LineOfSight, MilitaryMobilityClass, MobilityClass,
    RouteTrafficability, TerrainAnalyst, TerrainSnapshot,
};
use uuid::Uuid;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;
const DEMO_DURATION_SECS: u64 = 3600;

/// A terrain analyst that classifies by a simple slope heuristic.
struct SlopeTerrain {
    elevation: Vec<(f64, f64, f64)>, // (lat, lon, elev_m)
}

impl SlopeTerrain {
    fn new() -> Self {
        Self { elevation: vec![(DEMO_LAT, DEMO_LON, 150.0), (48.86, 2.36, 200.0)] }
    }

    fn slope_between(&self, lat: f64, lon: f64) -> f64 {
        // Approximate slope by distance-weighted nearest neighbour
        let (_, _, e) = self.elevation.iter().min_by(|a, b| {
            let da = (a.0 - lat).powi(2) + (a.1 - lon).powi(2);
            let db = (b.0 - lat).powi(2) + (b.1 - lon).powi(2);
            da.partial_cmp(&db).expect("float comparison should not produce NaN")
        }).unwrap_or(&(0.0, 0.0, 0.0));
        (e / 1000.0) * 100.0 // crude slope pct
    }
}

impl TerrainAnalyst for SlopeTerrain {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) {}

    fn mobility_class(&self, lat: f64, lon: f64) -> MobilityClass {
        let s = self.slope_between(lat, lon);
        if s < 10.0 { MobilityClass::Unrestricted }
        else if s < 20.0 { MobilityClass::Restricted }
        else if s < 30.0 { MobilityClass::SeverelyRestricted }
        else { MobilityClass::NoGo }
    }

    fn assess_route(&self, waypoints: &[(f64, f64)]) -> RouteTrafficability {
        let mut avg_speed = 40.0;
        for &(lat, lon) in waypoints {
            match self.mobility_class(lat, lon) {
                MobilityClass::Unrestricted => {}
                MobilityClass::Restricted => avg_speed *= 0.7,
                _ => avg_speed *= 0.3,
            }
        }
        RouteTrafficability {
            waypoints: waypoints.to_vec(), trafficable: avg_speed > 5.0,
            avg_speed_kph: avg_speed, bottlenecks: vec![],
        }
    }

    fn bridge_at(&self, _lat: f64, _lon: f64) -> Option<BridgeClassification> { None }

    fn line_of_sight(&self, from: (f64, f64, f64), to: (f64, f64, f64)) -> LineOfSight {
        let elev_from = self.slope_between(from.0, from.1) * 10.0;
        let elev_to = self.slope_between(to.0, to.1) * 10.0;
        LineOfSight { from, to, los: (from.2 > elev_from) && (to.2 > elev_to), masking_features: vec![] }
    }

    fn snapshot(&self) -> TerrainSnapshot {
        TerrainSnapshot {
            mobility_classes: self.elevation.iter().map(|&(lat, lon, _)| MilitaryMobilityClass {
                region_id: format!("{:.2},{:.2}", lat, lon),
                polygon: vec![(lat, lon)],
                classification: self.mobility_class(lat, lon),
                go_speed_kph: 40.0, no_go_speed_kph: 0.0,
            }).collect(),
            route_trafficability: vec![],
            bridges: vec![],
            los_constraints: vec![],
        }
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let terrain = SlopeTerrain::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    let scenario = Scenario { id: "t".into(), name: "Terrain Demo".into(), duration_secs: 60, order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({}) };
    let mut t = terrain;
    t.init(&scenario, &handle);

    println!("=== Terrain Analysis ===");
    println!(" ({:.2}, {:.2}) → {:?}", DEMO_LAT, DEMO_LON, t.mobility_class(DEMO_LAT, DEMO_LON));
    println!(" (48.90, 2.40) → {:?}", t.mobility_class(48.90, 2.40));

    let route = t.assess_route(&[(DEMO_LAT, DEMO_LON), (48.86, 2.36)]);
    println!(" Route trafficable: {} (avg {:.1} kph)", route.trafficable, route.avg_speed_kph);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_slope_is_restricted() {
        let t = SlopeTerrain::new();
        assert_eq!(t.mobility_class(DEMO_LAT, DEMO_LON), MobilityClass::Restricted);
    }

    #[test]
    fn test_route_is_trafficable() {
        let t = SlopeTerrain::new();
        let r = t.assess_route(&[(DEMO_LAT, DEMO_LON), (48.86, 2.36)]);
        assert!(r.trafficable);
    }

    #[test]
    fn test_terrain_has_mobility_classes_in_snapshot() {
        let t = SlopeTerrain::new();
        let s = t.snapshot();
        assert!(!s.mobility_classes.is_empty());
    }
}