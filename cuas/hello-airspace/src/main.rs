//! Demonstrates the `AirspaceManager` trait — defining no-fly zones,
//! restricted areas, and checking drone tracks against airspace volumes.

use furia_sdk::airspace_manager::AirspaceManager;
use furia_sdk::cuas::{AirspaceVolume, AirspaceVolumeType, DroneClass, DroneTrack};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use std::collections::HashMap;
use uuid::Uuid;

const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;

struct DemoAirspace {
    volumes: HashMap<String, AirspaceVolume>,
}

impl DemoAirspace {
    fn new() -> Self {
        Self { volumes: HashMap::new() }
    }
}

impl AirspaceManager for DemoAirspace {
    fn init(&mut self, _handle: &ModuleHandle) {}

    fn add_volume(&mut self, volume: AirspaceVolume) {
        self.volumes.insert(volume.volume_id.clone(), volume);
    }

    fn check(&self, track: &DroneTrack) -> Vec<AirspaceVolume> {
        self.volumes.values().filter(|v| {
            track.altitude_m >= v.min_altitude_m && track.altitude_m <= v.max_altitude_m
        }).cloned().collect()
    }

    fn is_in_no_fly_zone(&self, _lat: f64, _lon: f64, alt_m: f64) -> bool {
        self.volumes.values().any(|v| {
            v.volume_type == AirspaceVolumeType::NoFlyZone
                && alt_m >= v.min_altitude_m && alt_m <= v.max_altitude_m
        })
    }

    fn list_volumes(&self) -> Vec<AirspaceVolume> {
        self.volumes.values().cloned().collect()
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut airspace = DemoAirspace::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    airspace.add_volume(AirspaceVolume {
        volume_id: "NFZ-001".into(),
        volume_type: AirspaceVolumeType::NoFlyZone,
        polygon: vec![],
        min_altitude_m: 0.0,
        max_altitude_m: 1000.0,
        active_from_ms: 0,
        active_until_ms: u64::MAX,
        label: "Airbase Airspace".into(),
    });

    airspace.init(&handle);

    let track = DroneTrack {
        track_id: "DRONE-01".into(),
        lat: DEMO_LAT, lon: DEMO_LON, altitude_m: 500.0, speed_kph: 40.0, heading_deg: 90.0,
        drone_class: DroneClass::QuadcopterMedium, confidence: 0.85, sensor_type: "radar".into(),
        first_seen_ms: 0, last_seen_ms: 1000,
    };

    println!("=== Airspace Check ===");
    println!(" Track {} at {}m altitude", track.track_id, track.altitude_m);
    println!(" In no-fly zone: {}", airspace.is_in_no_fly_zone(track.lat, track.lon, track.altitude_m));
    println!(" Volumes affecting: {}", airspace.check(&track).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_volume(id: &str, vtype: AirspaceVolumeType) -> AirspaceVolume {
        AirspaceVolume {
            volume_id: id.into(), volume_type: vtype, polygon: vec![],
            min_altitude_m: 0.0, max_altitude_m: 1000.0,
            active_from_ms: 0, active_until_ms: u64::MAX, label: id.into(),
        }
    }

    fn make_track(alt: f64) -> DroneTrack {
        DroneTrack {
            track_id: "T1".into(), lat: 0.0, lon: 0.0, altitude_m: alt,
            speed_kph: 50.0, heading_deg: 0.0, drone_class: DroneClass::QuadcopterSmall,
            confidence: 0.9, sensor_type: "r".into(), first_seen_ms: 0, last_seen_ms: 0,
        }
    }

    #[test]
    fn test_add_and_list_volumes() {
        let mut a = DemoAirspace::new();
        a.add_volume(make_volume("V1", AirspaceVolumeType::NoFlyZone));
        assert_eq!(a.list_volumes().len(), 1);
    }

    #[test]
    fn test_no_fly_zone_detection() {
        let mut a = DemoAirspace::new();
        a.add_volume(make_volume("NFZ", AirspaceVolumeType::NoFlyZone));
        assert!(a.is_in_no_fly_zone(0.0, 0.0, 500.0));
        assert!(!a.is_in_no_fly_zone(0.0, 0.0, 2000.0));
    }

    #[test]
    fn test_check_returns_matching_volumes() {
        let mut a = DemoAirspace::new();
        a.add_volume(make_volume("V1", AirspaceVolumeType::Warning));
        let violations = a.check(&make_track(500.0));
        assert_eq!(violations.len(), 1);
    }
}
