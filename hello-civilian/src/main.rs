//! Demonstrates the `CivilianDensityProvider` trait — population by region.
//!
//! Civilian density models support IHL proportionality assessments by
//! estimating population distribution, time-of-day adjustments, civilian
//! movement, and collateral casualty estimates for a weapon delivery.

use std::time::Duration;

use furia_sdk::civilian_density::{CivilianDensityProvider, CivilianMovement, CivilianSnapshot, CollateralEstimate, PopulationRegion};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::Scenario;
use uuid::Uuid;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;
const DEMO_DURATION_SECS: u64 = 3600;

/// Population model for an urban area with time-of-day awareness.
struct UrbanPopulation {
    regions: Vec<PopulationRegion>,
}

impl UrbanPopulation {
    fn new() -> Self {
        Self {
            regions: vec![PopulationRegion {
                region_id: "downtown".into(),
                boundary: vec![(DEMO_LAT, DEMO_LON), (48.86, 2.36)],
                estimated_population: 50_000,
                density_per_km2: 5_000.0,
                time_of_day_multiplier: 0.8,
            }],
        }
    }
}

impl CivilianDensityProvider for UrbanPopulation {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) {}

    fn tick_population(&mut self, _dt: Duration) {}

    fn region_density(&self, region_id: &str) -> Option<PopulationRegion> {
        self.regions.iter().find(|r| r.region_id == region_id).cloned()
    }

    fn estimate_collateral(&self, _lat: f64, _lon: f64, weapon_radius_m: f64, weapon_type: &str) -> CollateralEstimate {
        let density = self.regions.first().map(|r| r.density_per_km2 * r.time_of_day_multiplier).unwrap_or(0.0);
        let kz_area_km2 = std::f64::consts::PI * (weapon_radius_m / 1000.0).powi(2);
        let estimated = (density * kz_area_km2).round() as u64;
        CollateralEstimate {
            weapon_type: weapon_type.into(),
            kz_radius_m: weapon_radius_m,
            pk_degraded: 0.15,
            estimated_civilians_in_kz: estimated,
            proportional_ok: estimated < 10,
        }
    }

    fn snapshot(&self) -> CivilianSnapshot {
        CivilianSnapshot {
            regions: self.regions.clone(),
            active_movements: vec![CivilianMovement {
                movement_id: "mov-1".into(), from_region: "downtown".into(),
                to_region: "suburb".into(), estimated_count: 500,
                confidence: 0.7, timeframe_ms: 3_600_000,
            }],
            risk_zones: vec![],
        }
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut pop = UrbanPopulation::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    let scenario = Scenario { id: "civ".into(), name: "Civilian Demo".into(), duration_secs: DEMO_DURATION_SECS, order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({}) };
    pop.init(&scenario, &handle);

    println!("=== Civilian Density ===");
    if let Some(r) = pop.region_density("downtown") {
        println!(" {}: {} ppl, {:.0}/km2", r.region_id, r.estimated_population, r.density_per_km2);
    }

    let ce = pop.estimate_collateral(DEMO_LAT + 0.005, DEMO_LON + 0.005, 50.0, "Mk82");
    println!(" Collateral estimate: {} civilians in KZ — proportional: {}", ce.estimated_civilians_in_kz, ce.proportional_ok);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_density_returns_some() {
        let p = UrbanPopulation::new();
        assert!(p.region_density("downtown").is_some());
    }

    #[test]
    fn test_collateral_small_weapon_proportional() {
        let p = UrbanPopulation::new();
        let ce = p.estimate_collateral(DEMO_LAT, DEMO_LON, 10.0, "small");
        assert!(ce.proportional_ok);
    }

    #[test]
    fn test_collateral_large_weapon_not_proportional() {
        let p = UrbanPopulation::new();
        let ce = p.estimate_collateral(DEMO_LAT, DEMO_LON, 500.0, "large");
        assert!(!ce.proportional_ok);
    }
}