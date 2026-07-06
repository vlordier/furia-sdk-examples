//! Demonstrates the `InterceptorPairingProvider` trait — pairing drone
//! threats with the best available interceptor asset.

use furia_sdk::cuas::{DroneClass, DroneTrack, EffectorKind, InterceptorAsset, InterceptorPairing, ThreatAssessment};
use furia_sdk::interceptor_pairing::InterceptorPairingProvider;
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use uuid::Uuid;

struct DemoPairingProvider {
    assets: Vec<InterceptorAsset>,
}

impl DemoPairingProvider {
    fn new() -> Self {
        Self { assets: vec![] }
    }
}

impl InterceptorPairingProvider for DemoPairingProvider {
    fn init(&mut self, assets: &[InterceptorAsset], _handle: &ModuleHandle) {
        self.assets = assets.to_vec();
    }

    fn pair(&self, track: &DroneTrack, _assessment: &ThreatAssessment) -> Vec<InterceptorPairing> {
        self.assets.iter().map(|asset| {
            let pk = match (&asset.effector_kind, &track.drone_class) {
                (EffectorKind::ShortRangeAgile, DroneClass::QuadcopterSmall) => 0.85,
                (EffectorKind::ShortRangeAgile, DroneClass::QuadcopterMedium) => 0.80,
                (EffectorKind::MediumRange, DroneClass::FixedWingSmall) => 0.80,
                (EffectorKind::MediumRange, DroneClass::FixedWingMedium) => 0.75,
                (EffectorKind::ElectronicAttack, DroneClass::QuadcopterSmall) => 0.90,
                _ => 0.50,
            };
            let range_km = ((track.lat - asset.lat).powi(2) + (track.lon - asset.lon).powi(2)).sqrt() * 111.0;
            InterceptorPairing {
                track_id: track.track_id.clone(),
                asset_id: asset.asset_id.clone(),
                pk,
                time_to_engage_s: range_km / 100.0 * 3600.0,
                recommended: pk >= 0.75 && asset.ready,
                rationale: format!("{} vs {:?}: Pk={:.2}", asset.name, track.drone_class, pk),
            }
        }).collect()
    }

    fn batch_pair(&self, tracks: &[DroneTrack], assessments: &[ThreatAssessment]) -> Vec<InterceptorPairing> {
        tracks.iter().zip(assessments.iter()).flat_map(|(t, a)| self.pair(t, a)).collect()
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let mut provider = DemoPairingProvider::new();
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    let assets = vec![
        InterceptorAsset {
            asset_id: "CIR-01".into(), name: "CIR-1".into(), effector_kind: EffectorKind::ShortRangeAgile,
            lat: 48.85, lon: 2.35, ready: true, rounds_remaining: 4, max_range_km: 15.0, min_range_km: 0.5, max_altitude_m: 1000.0,
        },
        InterceptorAsset {
            asset_id: "CIR-02".into(), name: "CIR-2".into(), effector_kind: EffectorKind::ElectronicAttack,
            lat: 48.86, lon: 2.36, ready: true, rounds_remaining: 10, max_range_km: 5.0, min_range_km: 0.1, max_altitude_m: 500.0,
        },
    ];
    provider.init(&assets, &handle);

    let track = DroneTrack {
        track_id: "DRONE-X".into(), lat: 48.90, lon: 2.40, altitude_m: 300.0, speed_kph: 40.0,
        heading_deg: 180.0, drone_class: DroneClass::QuadcopterSmall, confidence: 0.85,
        sensor_type: "rf".into(), first_seen_ms: 0, last_seen_ms: 2000,
    };

    let assessment = ThreatAssessment {
        track_id: track.track_id.clone(), drone_class: track.drone_class.clone(),
        threat_score: 0.7, threat_level: furia_sdk::platform::ThreatLevel::Medium,
        time_to_impact_s: None, distance_to_asset_km: 3.0, closest_asset: "CIR-01".into(),
        rationale: "Small quadcopter near perimeter".into(),
    };

    println!("=== Interceptor Pairings ===");
    for p in provider.pair(&track, &assessment) {
        println!(" {} → {} | Pk={:.2} | recommended={}", p.track_id, p.asset_id, p.pk, p.recommended);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_asset(id: &str, kind: EffectorKind) -> InterceptorAsset {
        InterceptorAsset {
            asset_id: id.into(), name: id.into(), effector_kind: kind,
            lat: 0.0, lon: 0.0, ready: true, rounds_remaining: 5,
            max_range_km: 10.0, min_range_km: 0.5, max_altitude_m: 1000.0,
        }
    }

    fn make_track(dc: DroneClass) -> DroneTrack {
        DroneTrack {
            track_id: "T1".into(), lat: 0.1, lon: 0.1, altitude_m: 200.0, speed_kph: 30.0,
            heading_deg: 0.0, drone_class: dc, confidence: 0.8, sensor_type: "r".into(),
            first_seen_ms: 0, last_seen_ms: 0,
        }
    }

    fn make_assessment() -> ThreatAssessment {
        ThreatAssessment {
            track_id: "T1".into(), drone_class: DroneClass::QuadcopterSmall, threat_score: 0.6,
            threat_level: furia_sdk::platform::ThreatLevel::Medium, time_to_impact_s: None,
            distance_to_asset_km: 2.0, closest_asset: "A1".into(), rationale: "test".into(),
        }
    }

    #[test]
    fn test_init_loads_assets() {
        let mut p = DemoPairingProvider::new();
        let h = ModuleHandle::new_test(Uuid::new_v4());
        p.init(&[make_asset("A1", EffectorKind::ShortRangeAgile)], &h);
        let track = make_track(DroneClass::QuadcopterSmall);
        let pairings = p.pair(&track, &make_assessment());
        assert_eq!(pairings.len(), 1);
        assert_eq!(pairings[0].asset_id, "A1");
    }

    #[test]
    fn test_quadcopter_small_gets_high_pk_from_short_range() {
        let mut p = DemoPairingProvider::new();
        let h = ModuleHandle::new_test(Uuid::new_v4());
        p.init(&[make_asset("A1", EffectorKind::ShortRangeAgile)], &h);
        let pairings = p.pair(&make_track(DroneClass::QuadcopterSmall), &make_assessment());
        assert!((pairings[0].pk - 0.85).abs() < 0.01);
    }

    #[test]
    fn test_batch_pair_returns_all_pairings() {
        let mut p = DemoPairingProvider::new();
        let h = ModuleHandle::new_test(Uuid::new_v4());
        p.init(&[make_asset("A1", EffectorKind::ShortRangeAgile), make_asset("A2", EffectorKind::ElectronicAttack)], &h);
        let tracks = vec![make_track(DroneClass::QuadcopterSmall)];
        let assessments = vec![make_assessment()];
        let pairings = p.batch_pair(&tracks, &assessments);
        assert_eq!(pairings.len(), 2);
    }
}
