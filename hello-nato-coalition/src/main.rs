//! Hello NATO/France Coalition example.
//!
//! Demonstrates coalition-labelled module lifecycle and audit using the
//! security/context types available in `furia-sdk` v0.1.0. Shared NATO domain
//! types are intentionally not imported because they are not in that git tag.

use furia_sdk::module_handle::{
    ClearanceLevel, LogLevel, ModuleHandle, ModuleHealth, SecurityContext,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CoalitionLabel {
    originator: &'static str,
    releasability: &'static str,
    caveat: &'static str,
}

impl CoalitionLabel {
    fn display(&self) -> String {
        format!(
            "{} / {} / {}",
            self.originator, self.releasability, self.caveat
        )
    }
}

fn main() {
    println!("Furia NATO Coalition Hello (SDK v0.1.0 lifecycle demo)");

    let ctx = SecurityContext {
        user_id: "fr-officer-01".into(),
        role: "commander".into(),
        session_id: "coalition-ex-42".into(),
        clearance: ClearanceLevel::Secret,
    };
    let label = CoalitionLabel {
        originator: "FR",
        releasability: "REL FR/UA",
        caveat: "DGA autonomy<=L3",
    };

    let handle = ModuleHandle::new_test(Uuid::new_v4());
    handle.log(LogLevel::Info, "hello-nato-coalition initialized");
    handle.health_report(ModuleHealth::Healthy);
    handle.audit("coalition.label", &label.display());

    println!(
        "Operator: {} / {} / {:?}",
        ctx.user_id, ctx.role, ctx.clearance
    );
    println!("Coalition label: {}", label.display());
    println!("Module initialized with id: {}", handle.module_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coalition_label_display_is_stable() {
        let label = CoalitionLabel {
            originator: "FR",
            releasability: "REL FR/UA",
            caveat: "DGA autonomy<=L3",
        };
        assert_eq!(label.display(), "FR / REL FR/UA / DGA autonomy<=L3");
    }

    #[test]
    fn sdk_security_context_is_available() {
        let ctx = SecurityContext {
            user_id: "fr-officer-01".into(),
            role: "commander".into(),
            session_id: "coalition-ex-42".into(),
            clearance: ClearanceLevel::Secret,
        };
        assert_eq!(ctx.role, "commander");
    }
}
