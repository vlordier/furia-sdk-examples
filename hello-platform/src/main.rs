//! Demonstrates composing multiple SDK providers into a "platform" using
//! a simple `ProviderRegistry` pattern.
//!
//! The registry stores boxed trait objects keyed by their type name,
//! allowing the platform to initialise them from a scenario, tick them
//! all in a loop, and inspect their health — mirroring how the real
//! Furia platform runtime works.

use std::time::Duration;

use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::{Scenario, SimulationProvider};

use uuid::Uuid;

/// A registry that holds providers and exposes common lifecycle operations.
struct ProviderRegistry {
    scenario: Scenario,
    sim_providers: Vec<(String, Box<dyn SimulationProvider>)>,
}

impl ProviderRegistry {
    fn new(scenario: Scenario) -> Self {
        Self { scenario, sim_providers: vec![] }
    }

    fn register_simulation(&mut self, name: &str, provider: Box<dyn SimulationProvider>, handle: &ModuleHandle) {
        let mut prov = provider;
        prov.init(&self.scenario, handle);
        self.sim_providers.push((name.to_string(), prov));
    }

    fn tick_all(&mut self, dt: Duration) {
        for (name, prov) in &mut self.sim_providers {
            let events = prov.tick(dt);
            println!("  [{}] tick: {} events", name, events.len());
        }
    }

    fn health_of(&self, name: &str) -> Option<ModuleHealth> {
        self.sim_providers.iter().find(|(n, _)| n == name).map(|(_, p)| p.health())
    }
}

/// A simple drone provider (minimal version for the platform example).
struct Drone {
    fuel: f64,
}

impl SimulationProvider for Drone {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) { self.fuel = 100.0; }
    fn tick(&mut self, dt: Duration) -> Vec<furia_sdk::simulation::SimEvent> {
        self.fuel -= 0.3 * dt.as_secs_f64();
        vec![]
    }
    fn entity_state(&self, _id: &str) -> Option<furia_sdk::simulation::EntityState> { None }
    fn health(&self) -> ModuleHealth {
        if self.fuel > 20.0 { ModuleHealth::Healthy } else { ModuleHealth::Degraded { reason: "low fuel".into() } }
    }
}

/// A second provider: a jammer simulation (also implementing SimulationProvider).
struct Jammer {
    active: bool,
}

impl SimulationProvider for Jammer {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) { self.active = true; }
    fn tick(&mut self, _dt: Duration) -> Vec<furia_sdk::simulation::SimEvent> { vec![] }
    fn entity_state(&self, _id: &str) -> Option<furia_sdk::simulation::EntityState> { None }
    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let scenario = Scenario {
        id: "platform-demo".into(), name: "Platform Demo".into(), duration_secs: 3600,
        order_of_battle: serde_json::json!({"uavs": ["drone-001"]}),
        timeline: vec![],
        environment: serde_json::json!({"wind_kph": 10}),
    };

    let mut registry = ProviderRegistry::new(scenario);
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    println!("=== ProviderRegistry — Composing Providers ===");
    registry.register_simulation("drone", Box::new(Drone { fuel: 0.0 }), &handle);
    registry.register_simulation("jammer", Box::new(Jammer { active: false }), &handle);

    println!("\n Providers registered:");
    for (name, _) in &registry.sim_providers {
        println!("  - {} | health: {:?}", name, registry.health_of(name));
    }

    println!("\n Tick cycle (2 ticks):");
    registry.tick_all(Duration::from_secs(60));
    registry.tick_all(Duration::from_secs(60));

    println!("\n Health after ticks:");
    println!("  drone  | health: {:?}", registry.health_of("drone"));
    println!("  jammer | health: {:?}", registry.health_of("jammer"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_can_register_providers() {
        let scenario = Scenario { id: "t".into(), name: "t".into(), duration_secs: 60, order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({}) };
        let mut reg = ProviderRegistry::new(scenario);
        let handle = ModuleHandle::new_test(Uuid::new_v4());
        reg.register_simulation("drone", Box::new(Drone { fuel: 0.0 }), &handle);
        assert!(reg.health_of("drone").is_some());
    }

    #[test]
    fn test_tick_all_does_not_panic() {
        let scenario = Scenario { id: "t".into(), name: "t".into(), duration_secs: 60, order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({}) };
        let mut reg = ProviderRegistry::new(scenario);
        let handle = ModuleHandle::new_test(Uuid::new_v4());
        reg.register_simulation("drone", Box::new(Drone { fuel: 100.0 }), &handle);
        reg.tick_all(Duration::from_secs(10)); // should not panic
    }

    #[test]
    fn test_health_degraded_after_many_ticks() {
        let scenario = Scenario { id: "t".into(), name: "t".into(), duration_secs: 60, order_of_battle: serde_json::json!({}), timeline: vec![], environment: serde_json::json!({}) };
        let mut reg = ProviderRegistry::new(scenario);
        let handle = ModuleHandle::new_test(Uuid::new_v4());
        reg.register_simulation("drone", Box::new(Drone { fuel: 100.0 }), &handle);
        // Simulate many ticks
        for _ in 0..10 {
            reg.tick_all(Duration::from_secs(60));
        }
        match reg.health_of("drone").unwrap() {
            ModuleHealth::Degraded { .. } => {} // expected
            _ => panic!("expected degraded after many ticks"),
        }
    }
}