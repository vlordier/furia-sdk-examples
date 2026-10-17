//! Demonstrates composing multiple SDK providers into a "platform" using
//! a simple `ProviderRegistry` pattern.
//!
//! The registry stores boxed trait objects keyed by their type name,
//! allowing the platform to initialise them from a scenario, tick them
//! all in a loop, and inspect their health — mirroring how the real
//! Furia platform runtime works.

use std::{collections::BTreeMap, time::Duration};

use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::simulation::{Scenario, SimulationProvider};

use uuid::Uuid;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_DURATION_SECS: u64 = 3600;

// Fuel burn rate: 0.3 = 30% per hour (simplified model for demo).
// Note: platform uses 0.3, simulation uses 0.5, logistics uses 0.3.
// These are intentionally different to demonstrate trait polymorphism
// across crate boundaries. In production, a shared SDK type would
// standardise fuel consumption modelling.

/// A registry that holds providers and exposes common lifecycle operations.
struct ProviderRegistry {
    scenario: Scenario,
    sim_providers: BTreeMap<String, Box<dyn SimulationProvider>>,
}

impl ProviderRegistry {
    fn new(scenario: Scenario) -> Self {
        Self {
            scenario,
            sim_providers: BTreeMap::new(),
        }
    }

    fn register_simulation(
        &mut self,
        name: &str,
        provider: Box<dyn SimulationProvider>,
        handle: &ModuleHandle,
    ) -> Result<(), String> {
        use std::collections::btree_map::Entry;

        match self.sim_providers.entry(name.to_owned()) {
            Entry::Occupied(_) => Err(format!("provider name already registered: {name}")),
            Entry::Vacant(entry) => {
                let mut provider = provider;
                provider.init(&self.scenario, handle)?;
                entry.insert(provider);
                Ok(())
            }
        }
    }

    fn tick_all(&mut self, dt: Duration) {
        for (name, prov) in &mut self.sim_providers {
            let events = prov.tick(dt);
            println!("  [{}] tick: {} events", name, events.len());
        }
    }

    fn health_of(&self, name: &str) -> Option<ModuleHealth> {
        self.sim_providers
            .get(name)
            .map(|provider| provider.health())
    }
}

/// A simple drone provider (minimal version for the platform example).
struct Drone {
    fuel: f64,
}

impl SimulationProvider for Drone {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) -> Result<(), String> {
        self.fuel = 100.0;
        Ok(())
    }
    fn tick(&mut self, dt: Duration) -> Vec<furia_sdk::simulation::SimEvent> {
        self.fuel -= 0.3 * dt.as_secs_f64();
        vec![]
    }
    fn entity_state(&self, _id: &str) -> Option<furia_sdk::simulation::EntityState> {
        None
    }
    fn health(&self) -> ModuleHealth {
        if self.fuel > 20.0 {
            ModuleHealth::Healthy
        } else {
            ModuleHealth::Degraded {
                reason: "low fuel".into(),
            }
        }
    }
}

/// A second provider: a jammer simulation (also implementing SimulationProvider).
struct Jammer {
    active: bool,
}

impl SimulationProvider for Jammer {
    fn init(&mut self, _scenario: &Scenario, _handle: &ModuleHandle) -> Result<(), String> {
        self.active = true;
        Ok(())
    }
    fn tick(&mut self, _dt: Duration) -> Vec<furia_sdk::simulation::SimEvent> {
        vec![]
    }
    fn entity_state(&self, _id: &str) -> Option<furia_sdk::simulation::EntityState> {
        None
    }
    fn health(&self) -> ModuleHealth {
        ModuleHealth::Healthy
    }
}

fn main() {
    let scenario = Scenario {
        id: "platform-demo".into(),
        name: "Platform Demo".into(),
        duration_secs: DEMO_DURATION_SECS,
        order_of_battle: serde_json::json!({"uavs": ["drone-001"]}),
        timeline: vec![],
        environment: serde_json::json!({"wind_kph": 10}),
    };

    let mut registry = ProviderRegistry::new(scenario);
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    println!("=== ProviderRegistry — Composing Providers ===");
    registry
        .register_simulation("drone", Box::new(Drone { fuel: 0.0 }), &handle)
        .expect("demo provider names must be unique");
    registry
        .register_simulation("jammer", Box::new(Jammer { active: false }), &handle)
        .expect("demo provider names must be unique");

    println!("\n Providers registered:");
    for name in registry.sim_providers.keys() {
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

    fn test_scenario() -> Scenario {
        Scenario {
            id: "t".into(),
            name: "t".into(),
            duration_secs: 60,
            order_of_battle: serde_json::json!({}),
            timeline: vec![],
            environment: serde_json::json!({}),
        }
    }

    #[test]
    fn test_registry_can_register_providers() {
        let mut reg = ProviderRegistry::new(test_scenario());
        let handle = ModuleHandle::new_test(Uuid::new_v4());
        reg.register_simulation("drone", Box::new(Drone { fuel: 0.0 }), &handle)
            .unwrap();
        assert!(reg.health_of("drone").is_some());
    }

    #[test]
    fn test_tick_all_does_not_panic() {
        let mut reg = ProviderRegistry::new(test_scenario());
        let handle = ModuleHandle::new_test(Uuid::new_v4());
        reg.register_simulation("drone", Box::new(Drone { fuel: 100.0 }), &handle)
            .unwrap();
        reg.tick_all(Duration::from_secs(10)); // should not panic
    }

    #[test]
    fn test_health_degraded_after_many_ticks() {
        let mut reg = ProviderRegistry::new(test_scenario());
        let handle = ModuleHandle::new_test(Uuid::new_v4());
        reg.register_simulation("drone", Box::new(Drone { fuel: 100.0 }), &handle)
            .unwrap();
        // Simulate many ticks
        for _ in 0..10 {
            reg.tick_all(Duration::from_secs(60));
        }
        assert!(matches!(
            reg.health_of("drone").expect("drone should be registered"),
            ModuleHealth::Degraded { .. }
        ));
    }

    #[test]
    fn registry_rejects_duplicate_names_without_replacing_the_registered_provider() {
        let mut registry = ProviderRegistry::new(test_scenario());
        let handle = ModuleHandle::new_test(Uuid::new_v4());
        registry
            .register_simulation("shared", Box::new(Drone { fuel: 0.0 }), &handle)
            .unwrap();
        let duplicate =
            registry.register_simulation("shared", Box::new(Jammer { active: false }), &handle);

        assert_eq!(
            duplicate,
            Err("provider name already registered: shared".into())
        );
        registry.tick_all(Duration::from_secs(300));
        assert!(matches!(
            registry.health_of("shared"),
            Some(ModuleHealth::Degraded { .. })
        ));
        assert_eq!(registry.health_of("missing"), None);
    }
}
