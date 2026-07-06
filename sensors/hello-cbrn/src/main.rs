//! Demonstrates the `CbrnProvider` trait — Gaussian plume dispersion.
//!
//! CBRN simulation models the release of chemical, biological, or
//! radiological agents and their downwind dispersion as a Gaussian plume.
//! The module tracks contamination zones, hazard levels, and affected units.

use std::time::Duration;

use furia_sdk::cbrn::{CbrnHazardLevel, CbrnProvider, CbrnRelease, CbrnSnapshot, ContaminationZone, DownwindHazard};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::{Scenario, SimEvent};
use uuid::Uuid;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;
const DEMO_DURATION_SECS: u64 = 3600;

/// A Gaussian plume dispersion simulator.
struct GaussianPlume {
    releases: Vec<CbrnRelease>,
    hazards: Vec<DownwindHazard>,
    t_elapsed_s: f64,
}

impl GaussianPlume {
    fn new() -> Self { Self { releases: vec![], hazards: vec![], t_elapsed_s: 0.0 } }
}

impl CbrnProvider for GaussianPlume {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) {}

    fn release_agent(&mut self, release: CbrnRelease) -> Vec<SimEvent> {
        self.releases.push(release.clone());
        let hazard = DownwindHazard {
            release_id: release.release_id.clone(),
            hazard_zone: vec![(release.lat, release.lon), (release.lat + 0.01, release.lon + 0.01)],
            arrival_time_s: 120.0,
            concentration_pct: 0.8,
            hazard_level: CbrnHazardLevel::ImmediateDanger,
        };
        self.hazards.push(hazard);
        vec![SimEvent {
            event_type: "cbrn_release".into(), source: release.release_id.clone(),
            target: None, params: serde_json::json!({"agent": release.agent_type, "mass_kg": release.mass_kg}),
            timestamp_ms: release.time_utc_ms,
        }]
    }

    fn tick_dispersion(&mut self, dt: Duration) -> Vec<SimEvent> {
        self.t_elapsed_s += dt.as_secs_f64();
        for h in &mut self.hazards {
            h.concentration_pct = (h.concentration_pct * 0.99).max(0.0);
        }
        vec![]
    }

    fn hazard_at(&self, lat: f64, lon: f64) -> Option<CbrnHazardLevel> {
        for h in &self.hazards {
            for &(hz_lat, hz_lon) in &h.hazard_zone {
                let d = ((lat - hz_lat).powi(2) + (lon - hz_lon).powi(2)).sqrt();
                if d < 0.02 {
                    return Some(h.hazard_level.clone());
                }
            }
        }
        None
    }

    fn snapshot(&self) -> CbrnSnapshot {
        CbrnSnapshot {
            active_releases: self.releases.clone(),
            hazard_zones: self.hazards.clone(),
            contamination_zones: self.hazards.iter().map(|h| ContaminationZone {
                zone_id: format!("cz-{}", h.release_id),
                polygon: h.hazard_zone.clone(),
                agent_type: "sarin".into(),
                contamination_pct: h.concentration_pct,
                decay_hours: 12.0,
            }).collect(),
            affected_units: vec![],
        }
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut cbrn = GaussianPlume::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    let scenario = Scenario { id: "cbrn".into(), name: "CBRN Demo".into(), duration_secs: DEMO_DURATION_SECS, order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({"wind_dir_deg": 270, "wind_speed_kph": 15}) };
    cbrn.init(&scenario, &handle);

    println!("=== CBRN Dispersion ===");
    let _ev = cbrn.release_agent(CbrnRelease {
        release_id: "rel-001".into(), agent_type: "sarin".into(),
        lat: DEMO_LAT, lon: DEMO_LON, altitude_m: 10.0, mass_kg: 50.0, time_utc_ms: 0,
    });
    cbrn.tick_dispersion(Duration::from_secs(300));

    println!(" Hazard near release: {:?}", cbrn.hazard_at(48.86, 2.36));
    println!(" Hazard far away: {:?}", cbrn.hazard_at(50.0, 3.0));
    println!(" Active releases: {}", cbrn.snapshot().active_releases.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_creates_hazard() {
        let mut c = GaussianPlume::new();
        c.release_agent(CbrnRelease { release_id: "r1".into(), agent_type: "gas".into(), lat: 0.0, lon: 0.0, altitude_m: 0.0, mass_kg: 10.0, time_utc_ms: 0 });
        assert_eq!(c.hazards.len(), 1);
    }

    #[test]
    fn test_hazard_at_nearby_returns_level() {
        let mut c = GaussianPlume::new();
        c.release_agent(CbrnRelease { release_id: "r1".into(), agent_type: "gas".into(), lat: DEMO_LAT, lon: DEMO_LON, altitude_m: 0.0, mass_kg: 10.0, time_utc_ms: 0 });
        assert_eq!(c.hazard_at(48.86, 2.36), Some(CbrnHazardLevel::ImmediateDanger));
    }

    #[test]
    fn test_hazard_at_distant_returns_none() {
        let c = GaussianPlume::new();
        assert_eq!(c.hazard_at(50.0, 3.0), None);
    }
}