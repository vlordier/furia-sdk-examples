//! Demonstrates the `SimulationProvider` trait — a drone flying a patrol route
//! with simulated fuel consumption, position updates, and health reporting.
//!
//! `SimulationProvider` is the core simulation runtime trait. It initialises
//! from a `Scenario`, advances via `tick()`, exposes per-entity state, and
//! reports health. Every Furia simulator module implements this trait.

use std::time::Duration;

use furia_sdk::module_handle::{LogLevel, ModuleHandle, ModuleHealth};
use furia_sdk::simulation::{EntityState, Scenario, SimEvent, SimulationProvider};
use uuid::Uuid;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;
const DEMO_DURATION_SECS: u64 = 3600;

// Fuel burn rate: 0.5 = 50% per second (simplified model for demo).
// Note: simulation uses 0.5, logistics uses 0.3, platform uses 0.3.
// These are intentionally different to demonstrate trait polymorphism
// across crate boundaries. In production, a shared SDK type would
// standardise fuel consumption modelling.

/// A minimal drone simulator that patrols a route and burns fuel.
struct PatrolDrone {
    handle: Option<ModuleHandle>,
    fuel_liters: f64,
    position: (f64, f64, f64),
    tick_count: u64,
}

impl PatrolDrone {
    fn new() -> Self {
        Self { handle: None, fuel_liters: 100.0, position: (DEMO_LAT, DEMO_LON, 500.0), tick_count: 0 }
    }
}

impl SimulationProvider for PatrolDrone {
    fn init(&mut self, scenario: &Scenario, handle: &ModuleHandle) {
        self.handle = Some(ModuleHandle::new_test(handle.module_id));
        self.handle.as_ref().expect("init must be called before tick").log(LogLevel::Info, &format!("DroneSim: scenario '{}' loaded", scenario.name));
    }

    fn tick(&mut self, dt: Duration) -> Vec<SimEvent> {
        self.tick_count += 1;
        self.fuel_liters -= 0.5 * dt.as_secs_f64();
        self.position.1 += 0.001; // drift east each tick

        if self.fuel_liters <= 0.0 {
            return vec![SimEvent {
                event_type: "critical".into(),
                source: "drone-001".into(),
                target: Some("base".into()),
                params: serde_json::json!({"reason": "out of fuel"}),
                timestamp_ms: self.tick_count * 1000,
            }];
        }
        vec![SimEvent {
            event_type: "tick".into(),
            source: "drone-001".into(),
            target: None,
            params: serde_json::json!({"fuel": self.fuel_liters}),
            timestamp_ms: self.tick_count * 1000,
        }]
    }

    fn entity_state(&self, entity_id: &str) -> Option<EntityState> {
        if entity_id == "drone-001" {
            Some(EntityState {
                entity_id: "drone-001".into(),
                position: self.position,
                velocity: Some(25.0),
                heading: Some(90.0),
                status: if self.fuel_liters > 0.0 { "patrolling".into() } else { "bingo".into() },
            })
        } else {
            None
        }
    }

    fn health(&self) -> ModuleHealth {
        if self.fuel_liters > 20.0 {
            ModuleHealth::Healthy
        } else if self.fuel_liters > 0.0 {
            ModuleHealth::Degraded { reason: format!("fuel {:.1}L", self.fuel_liters) }
        } else {
            ModuleHealth::Unhealthy { reason: "fuel depleted".into() }
        }
    }
}

fn main() {
    let scenario = Scenario {
        id: "scenario-001".into(),
        name: "Drone Patrol Alpha".into(),
        duration_secs: DEMO_DURATION_SECS,
        order_of_battle: serde_json::json!({"drones": ["drone-001"]}),
        timeline: vec![],
        environment: serde_json::json!({"wind_kph": 15}),
    };

    let mut drone = PatrolDrone::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());
    drone.init(&scenario, &handle);

    println!("=== Drone Patrol Simulation ===");
    for _ in 0..3 {
        let _events = drone.tick(Duration::from_secs(60));
        if let Some(state) = drone.entity_state("drone-001") {
            println!(" Position: ({:.4}, {:.4}, {:.0}) — {}", state.position.0, state.position.1, state.position.2, state.status);
        }
        println!(" Health: {:?}", drone.health());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drone_init_and_state() {
        let mut drone = PatrolDrone::new();
        let scenario = Scenario {
            id: "test".into(), name: "test".into(), duration_secs: 60,
            order_of_battle: serde_json::json!({}), timeline: vec![],
            environment: serde_json::json!({}),
        };
        drone.init(&scenario, &ModuleHandle::new_test(Uuid::new_v4()));
        let state = drone.entity_state("drone-001").expect("drone-001 should be registered");
        assert_eq!(state.entity_id, "drone-001");
        assert_eq!(state.status, "patrolling");
    }

    #[test]
    fn test_tick_burns_fuel() {
        let mut drone = PatrolDrone::new();
        drone.init(&Scenario {
            id: "test".into(), name: "test".into(), duration_secs: 60,
            order_of_battle: serde_json::json!({}), timeline: vec![],
            environment: serde_json::json!({}),
        }, &ModuleHandle::new_test(Uuid::new_v4()));
        drone.tick(Duration::from_secs(60));
        assert!(drone.fuel_liters < 100.0);
    }

    #[test]
    fn test_health_bingo_on_low_fuel() {
        let mut drone = PatrolDrone::new();
        drone.fuel_liters = 10.0;
        assert!(matches!(drone.health(), ModuleHealth::Degraded { .. }));
    }
}