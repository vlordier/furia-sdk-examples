//! Demonstrates the `LogisticsProvider` trait — tracking convoy fuel burn.
//!
//! Logistics providers model fuel and ammunition consumption over time,
//! supply chain throughput, and vehicle maintenance. This example tracks
//! a single convoy's fuel use across a route.

use std::collections::HashMap;
use std::time::Duration;

use furia_sdk::logistics::{ConvoyState, LogisticsProvider, LogisticsSnapshot, SupplyChainNode, VehicleLogistics};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::{Scenario, SimEvent};
use uuid::Uuid;

/// A logistics simulator that burns fuel for a single convoy.
struct ConvoyLogistics {
    vehicles: HashMap<String, VehicleLogistics>,
    convoys: Vec<ConvoyState>,
}

impl ConvoyLogistics {
    fn new() -> Self {
        let mut vehicles = HashMap::new();
        vehicles.insert("truck-001".into(), VehicleLogistics {
            vehicle_id: "truck-001".into(), fuel_liters: 200.0, max_fuel_liters: 200.0,
            ammo_remaining: 500.0, ammo_capacity: 500.0, maintenance_hours: 0.0, health_pct: 100.0,
        });
        Self {
            vehicles,
            convoys: vec![ConvoyState {
                convoy_id: "convoy-a".into(),
                vehicles: vec!["truck-001".into()],
                position: (48.85, 2.35), speed_kph: 40.0,
                destination_id: "base-bravo".into(), fuel_consumed_liters: 0.0,
            }],
        }
    }
}

impl LogisticsProvider for ConvoyLogistics {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) {}

    fn tick_consumption(&mut self, dt: Duration) -> Vec<SimEvent> {
        let mut events = vec![];
        for convoy in &mut self.convoys {
            let burn = convoy.speed_kph * dt.as_secs_f64() / 3600.0 * 0.3; // 0.3 L/km
            convoy.fuel_consumed_liters += burn;
            if let Some(v) = self.vehicles.get_mut("truck-001") {
                v.fuel_liters = (v.fuel_liters - burn).max(0.0);
                if v.fuel_liters < 20.0 {
                    events.push(SimEvent {
                        event_type: "low_fuel".into(), source: "truck-001".into(),
                        target: Some("convoy-a".into()),
                        params: serde_json::json!({"fuel_liters": v.fuel_liters}),
                        timestamp_ms: 0,
                    });
                }
            }
        }
        events
    }

    fn vehicle_state(&self, vehicle_id: &str) -> Option<VehicleLogistics> {
        self.vehicles.get(vehicle_id).cloned()
    }

    fn supply_node(&self, _node_id: &str) -> Option<SupplyChainNode> { None }

    fn snapshot(&self) -> LogisticsSnapshot {
        LogisticsSnapshot {
            vehicle_states: self.vehicles.values().cloned().collect(),
            supply_nodes: vec![],
            convoys: self.convoys.clone(),
            critical_shortages: vec![],
        }
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut logi = ConvoyLogistics::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    let scenario = Scenario { id: "logi-test".into(), name: "Logi Demo".into(), duration_secs: 3600, order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({}) };
    logi.init(&scenario, &handle);

    println!("=== Logistics Simulation ===");
    for _ in 0..5 {
        let events = logi.tick_consumption(Duration::from_secs(300));
        let v = logi.vehicle_state("truck-001").unwrap();
        println!(" truck-001 fuel: {:.1}L — events: {}", v.fuel_liters, events.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_fuel() {
        let l = ConvoyLogistics::new();
        let v = l.vehicle_state("truck-001").unwrap();
        assert_eq!(v.fuel_liters, 200.0);
    }

    #[test]
    fn test_tick_burns_fuel() {
        let mut l = ConvoyLogistics::new();
        l.init(&Scenario { id: "t".into(), name: "t".into(), duration_secs: 60, order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({}) }, &ModuleHandle::new_test(Uuid::new_v4()));
        l.tick_consumption(Duration::from_secs(600));
        let v = l.vehicle_state("truck-001").unwrap();
        assert!(v.fuel_liters < 200.0);
    }

    #[test]
    fn test_snapshot_includes_vehicle() {
        let l = ConvoyLogistics::new();
        let s = l.snapshot();
        assert_eq!(s.vehicle_states.len(), 1);
    }
}