//! Demonstrates the `IntentProvider` trait — parsing commander intent text.
//!
//! Intent providers parse natural-language (or structured) commander
//! directives into a structured `CommanderIntent` that drives downstream
//! mission planning, constraint generation, and policy evaluation.

use furia_sdk::intent::{CommanderIntent, IntentParseError, IntentProvider};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use uuid::Uuid;

/// A simple keyword-based intent parser.
struct TextIntentParser;

impl IntentProvider for TextIntentParser {
    fn parse_intent(&self, input: &str, _handle: &ModuleHandle) -> Result<CommanderIntent, IntentParseError> {
        let parts: Vec<&str> = input.split(|c| c == ' ' || c == ',').filter(|s| !s.is_empty()).collect();
        let objective = if input.to_lowercase().contains("destroy") {
            "destroy".into()
        } else if input.to_lowercase().contains("recon") {
            "reconnaissance".into()
        } else {
            "unknown".into()
        };

        let mut constraints = vec![];
        if input.to_lowercase().contains("collateral") {
            constraints.push("no_collateral_damage".into());
        }
        if input.contains("NLT") {
            constraints.push("time_constrained".into());
        }

        let target: Option<String> = parts.iter().position(|&p| p.eq_ignore_ascii_case("at"))
            .and_then(|i| parts.get(i + 1))
            .map(|s| s.to_string());

        Ok(CommanderIntent {
            mission_id: "intent-001".into(),
            objective,
            constraints,
            target,
        })
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let parser = TextIntentParser;
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    let inputs = vec![
        "destroy T72 at grid-48-85, collateral minimal",
        "recon route to bravo, NLT 1400Z",
    ];
    println!("=== Intent Parsing ===");
    for input in inputs {
        match parser.parse_intent(input, &handle) {
            Ok(_intent) => println!("\n Input: \"{}\"", input),
            Err(e) => println!(" Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_destroy_intent() {
        let p = TextIntentParser;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let intent = p.parse_intent("destroy tank at grid-alpha", &h).unwrap();
        assert_eq!(intent.objective, "destroy");
    }

    #[test]
    fn test_parse_recon_intent() {
        let p = TextIntentParser;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let intent = p.parse_intent("recon route to bravo", &h).unwrap();
        assert_eq!(intent.objective, "reconnaissance");
    }

    #[test]
    fn test_collateral_constraint_detected() {
        let p = TextIntentParser;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let intent = p.parse_intent("destroy tank, collateral minimal", &h).unwrap();
        assert!(intent.constraints.contains(&"no_collateral_damage".into()));
    }
}