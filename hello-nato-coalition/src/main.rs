//! Hello NATO/France Coalition example.
//! Demonstrates a basic coalition-aware provider stub using available SDK types.
//!
//! Note: Full NATO types (Nation, MarkingProfile, NationalCaveat) are defined
//! in the furia-sdk crate. This example shows the pattern for a coalition-aware
//! module using furia-sdk.

use furia_sdk::module_handle::{ModuleHandle, SecurityContext, ClearanceLevel, ModuleHealth, LogLevel};
use uuid::Uuid;

fn main() {
    println!("Furia NATO Coalition Hello (SDK example)");

    // Demonstrate ModuleHandle with a security context
    let ctx = SecurityContext {
        user_id: "fr-officer-01".into(),
        role: "commander".into(),
        session_id: "coalition-ex-42".into(),
        clearance: ClearanceLevel::Secret,
    };
    println!("Security context: {} / {} / {:?}",
        ctx.user_id, ctx.role, ctx.clearance);

    let handle = ModuleHandle::new_test(Uuid::new_v4());
    handle.log(LogLevel::Info, "hello-nato-coalition initialized");
    handle.health_report(ModuleHealth::Healthy);
    handle.audit("module.init", "hello-nato-coalition");

    println!("Module initialized with id: {}", handle.module_id);
    println!();
    println!("To extend: add CoalitionProvider trait to furia-sdk and");
    println!("implement multi-nation types (Nation, MarkingProfile, NationalCaveat)");
    println!("in a shared crate (furia-core) for full coalition C2.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_can_create_handle() {
        let handle = ModuleHandle::new_test(Uuid::new_v4());
        assert!(!handle.module_id.is_nil());
    }
}