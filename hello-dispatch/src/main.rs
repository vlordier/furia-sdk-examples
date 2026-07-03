//! Demonstrates the `DispatchAdapter` trait — firing an action at a target.
//!
//! The dispatch adapter handles outbound command delivery to external
//! platforms. It validates targets for reachability, then dispatches
//! actions and returns a `DispatchReceipt`.

use furia_sdk::dispatch::{Action, DispatchAdapter, DispatchReceipt, TargetValidation};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::sensor::{AdapterError, TrackIngest};
use uuid::Uuid;

/// A mock dispatch adapter simulating a weapon release.
struct WeaponDispatch;

impl DispatchAdapter for WeaponDispatch {
    fn dispatch(&self, action: &Action, _handle: &ModuleHandle) -> Result<DispatchReceipt, AdapterError> {
        Ok(DispatchReceipt {
            dispatch_id: format!("disp-{}", &action.action_id),
            target_id: action.target.track_id.clone(),
            timestamp: chrono::Utc::now(),
            ack_status: "delivered".into(),
            adapter_specific: serde_json::json!({"protocol": action.dispatch_protocol}),
        })
    }

    fn validate_target(&self, target: &TrackIngest, _handle: &ModuleHandle) -> Result<TargetValidation, AdapterError> {
        if target.longitude > -180.0 && target.longitude < 180.0 {
            Ok(TargetValidation::Reachable)
        } else {
            Ok(TargetValidation::Unreachable { reason: "invalid coordinates".into() })
        }
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let dispatch = WeaponDispatch;
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    let target = TrackIngest {
        track_id: "threat-t72".into(), latitude: 48.85, longitude: 2.35,
        altitude: Some(0.0), velocity: Some(10.0), heading: Some(45.0),
        classification: Some("hostile".into()), confidence: 0.9, sensor_type: "radar".into(),
    };

    println!("=== Target Validation ===");
    println!(" {:?}", dispatch.validate_target(&target, &handle));

    let action = Action {
        action_id: "act-001".into(), action_class: "strike".into(),
        target, dispatch_protocol: "cot".into(), payload: vec![0x01, 0x02],
    };

    println!("=== Dispatch ===");
    let receipt = dispatch.dispatch(&action, &handle).unwrap();
    println!(" Receipt: ack={}, time={}", receipt.ack_status, receipt.timestamp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_target_reachable() {
        let t = TrackIngest {
            track_id: "t1".into(), latitude: 0.0, longitude: 10.0,
            altitude: None, velocity: None, heading: None,
            classification: None, confidence: 1.0, sensor_type: "test".into(),
        };
        let d = WeaponDispatch;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        assert_eq!(d.validate_target(&t, &h).unwrap(), TargetValidation::Reachable);
    }

    #[test]
    fn test_dispatch_returns_receipt() {
        let d = WeaponDispatch;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let t = TrackIngest { track_id: "t1".into(), latitude: 0.0, longitude: 0.0, altitude: None, velocity: None, heading: None, classification: None, confidence: 1.0, sensor_type: "t".into() };
        let a = Action { action_id: "a1".into(), action_class: "test".into(), target: t, dispatch_protocol: "mock".into(), payload: vec![] };
        let r = d.dispatch(&a, &h).unwrap();
        assert_eq!(r.ack_status, "delivered");
    }

    #[test]
    fn test_health_is_ok() {
        assert_eq!(WeaponDispatch.health(), ModuleHealth::Healthy);
    }
}