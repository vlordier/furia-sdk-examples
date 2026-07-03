//! Demonstrates the `PolicyProvider` trait — IHL and ROE evaluation.
//!
//! A simple policy module that denies engagement of civilians (Distinction
//! principle) and always allows action against confirmed threats. The
//! `evaluate()` method receives a `PolicyContext` and returns a
//! `PolicyDecision`.

use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::policy::{PolicyConstraint, PolicyContext, PolicyDecision, PolicyError, PolicyProvider};
use uuid::Uuid;

/// A rules-based policy provider enforcing basic IHL principles.
struct CivilianProtectionPolicy;

impl PolicyProvider for CivilianProtectionPolicy {
    fn evaluate(&self, ctx: &PolicyContext, _handle: &ModuleHandle) -> Result<PolicyDecision, PolicyError> {
        // Civilian targets always denied (Distinction principle)
        if ctx.target_id.starts_with("civ-") {
            return Ok(PolicyDecision::Deny {
                reason: "civilian target — violates Distinction".into(),
                ihl_articles: vec!["API 51".into(), "API 52".into()],
            });
        }
        // Confirmed threat with high enough confidence
        if ctx.target_id.starts_with("threat-") && ctx.target_confidence >= 0.8 {
            return Ok(PolicyDecision::Allow);
        }
        // Marginal cases escalate
        Ok(PolicyDecision::Escalate {
            reason: format!("confidence {:.2} below threshold", ctx.target_confidence),
            recommended_approver: "Tactical Director".into(),
        })
    }

    fn intent_feedback(&self, _intent: &str) -> Vec<PolicyConstraint> {
        vec![PolicyConstraint {
            zone: Some("no-strike-zone-A".into()),
            max_collateral_risk: Some(0.1),
            min_confidence: Some(0.85),
        }]
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let policy = CivilianProtectionPolicy;
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    let civilian_ctx = PolicyContext {
        action_class: "engage".into(), target_id: "civ-001".into(),
        target_confidence: 0.95, collateral_risk_score: 0.0,
        operator_role: "pilot".into(), mission_id: "m-001".into(),
    };
    let threat_ctx = PolicyContext {
        action_class: "engage".into(), target_id: "threat-t72".into(),
        target_confidence: 0.92, collateral_risk_score: 0.05,
        operator_role: "pilot".into(), mission_id: "m-001".into(),
    };

    println!("=== Civilian Target ===");
    println!(" Decision: {:?}", policy.evaluate(&civilian_ctx, &handle));

    println!("=== Confirmed Threat ===");
    println!(" Decision: {:?}", policy.evaluate(&threat_ctx, &handle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civilian_denied() {
        let p = CivilianProtectionPolicy;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let ctx = PolicyContext {
            action_class: "engage".into(), target_id: "civ-001".into(),
            target_confidence: 0.9, collateral_risk_score: 0.0,
            operator_role: "pilot".into(), mission_id: "m-001".into(),
        };
        let r = p.evaluate(&ctx, &h).unwrap();
        assert_eq!(r, PolicyDecision::Deny { reason: "civilian target — violates Distinction".into(), ihl_articles: vec!["API 51".into(), "API 52".into()] });
    }

    #[test]
    fn test_threat_allowed() {
        let p = CivilianProtectionPolicy;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let ctx = PolicyContext {
            action_class: "engage".into(), target_id: "threat-t72".into(),
            target_confidence: 0.92, collateral_risk_score: 0.05,
            operator_role: "pilot".into(), mission_id: "m-001".into(),
        };
        assert_eq!(p.evaluate(&ctx, &h).unwrap(), PolicyDecision::Allow);
    }

    #[test]
    fn test_low_confidence_escalates() {
        let p = CivilianProtectionPolicy;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let ctx = PolicyContext {
            action_class: "engage".into(), target_id: "threat-t72".into(),
            target_confidence: 0.5, collateral_risk_score: 0.05,
            operator_role: "pilot".into(), mission_id: "m-001".into(),
        };
        assert!(matches!(p.evaluate(&ctx, &h).unwrap(), PolicyDecision::Escalate { .. }));
    }
}