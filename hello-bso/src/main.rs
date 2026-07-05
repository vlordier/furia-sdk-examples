//! # Hello BattlespaceObject
//!
//! Demonstrates constructing, querying, and serialising the authoritative
//! [`BattlespaceObject`] type from `furia_sdk::cop`.
//!
//! `BattlespaceObject` is the single, normalised track representation shared
//! across all of Furia's data paths — radar ingest, CoT from ATAK, STANAG 4586
//! external vehicles, SAPIENT sensor-fusion — all converge on this type before
//! reaching the UI or the kill-chain.
//!
//! ## What this example shows
//!
//! 1. **Constructing** objects with the mandatory fields (`id`, `affiliation`,
//!    position, `track_quality`, `pedigree`).
//! 2. **Using `Affiliation` helpers** — `is_hostile()`, `is_friend()`, etc.
//! 3. **Building a mini COP snapshot** with multiple objects.
//! 4. **Filtering** for threats vs friendlies.
//! 5. **Serialising** to JSON — same shape the cop-service REST API returns.
//!
//! Run with:
//!
//! ```text
//! cargo run -p hello-bso
//! ```

use furia_sdk::cop::{Affiliation, BattlespaceObject, ErrorEllipse, SourcePedigree, Velocity};
use uuid::Uuid;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn make_pedigree(system: &str, original_id: &str) -> SourcePedigree {
    SourcePedigree {
        system: system.to_owned(),
        original_id: original_id.to_owned(),
        classification_label: "NATO UNCLASSIFIED".to_owned(),
        releasability: vec!["NATO".to_owned()],
        observed_at_ms: 1_700_000_000_000,
    }
}

fn make_object(
    label: &str,
    lat: f64,
    lon: f64,
    affiliation: Affiliation,
    system: &str,
) -> BattlespaceObject {
    BattlespaceObject {
        id: Uuid::new_v4().to_string(),
        affiliation,
        latitude: lat,
        longitude: lon,
        altitude_m: Some(1_500.0),
        error_ellipse: Some(ErrorEllipse {
            semi_major_m: 50.0,
            semi_minor_m: 30.0,
            orientation_deg: 45.0,
            confidence: 0.95,
        }),
        track_quality: furia_sdk::cop::TrackQuality(12),
        velocity_ms: Some(Velocity {
            north_ms: 0.0,
            east_ms: 50.0,
            down_ms: 0.0,
        }),
        sidc: None,
        label: Some(label.to_owned()),
        pedigree: make_pedigree(system, label),
        updated_at_ms: 1_700_000_001_000,
        attributes: std::collections::HashMap::new(),
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    // ── 1. Build a small COP picture ─────────────────────────────────────────
    let objects: Vec<BattlespaceObject> = vec![
        make_object("ALPHA-1", 48.85, 2.35, Affiliation::Friend, "mavlink"),
        make_object("ALPHA-2", 48.86, 2.36, Affiliation::AssumedFriend, "stanag4586"),
        make_object("BRAVO-1", 48.83, 2.32, Affiliation::Hostile, "sapient"),
        make_object("CHARLIE-1", 48.80, 2.40, Affiliation::Suspect, "asterix"),
        make_object("DELTA-1", 48.90, 2.50, Affiliation::Neutral, "tak-cot"),
        make_object("ECHO-1", 48.95, 2.55, Affiliation::Civilian, "manual"),
    ];

    println!("=== COP Snapshot ({} objects) ===\n", objects.len());
    for obj in &objects {
        let label = obj.label.as_deref().unwrap_or(&obj.id);
        println!(
            "  {:10}  affiliation={:?}  source={:10}  tq={:2}  threat={}  friend={}",
            label,
            obj.affiliation,
            obj.pedigree.system,
            obj.track_quality.0,
            obj.affiliation.is_threat(),
            obj.affiliation.is_friend(),
        );
    }

    // ── 2. Filter threats ─────────────────────────────────────────────────────
    let threats: Vec<&BattlespaceObject> =
        objects.iter().filter(|o| o.affiliation.is_threat()).collect();

    println!("\n=== Threat tracks ({}) ===", threats.len());
    for t in &threats {
        println!(
            "  {} at ({:.4}°N, {:.4}°E)  tq={}",
            t.label.as_deref().unwrap_or(&t.id),
            t.latitude,
            t.longitude,
            t.track_quality.0,
        );
    }

    // ── 3. Filter friendlies ──────────────────────────────────────────────────
    let friendlies: Vec<&BattlespaceObject> =
        objects.iter().filter(|o| o.affiliation.is_friend()).collect();

    println!("\n=== Friendly tracks ({}) ===", friendlies.len());
    for f in &friendlies {
        println!(
            "  {} ({})",
            f.label.as_deref().unwrap_or(&f.id),
            f.pedigree.system,
        );
    }

    // ── 4. Serialise to JSON (same shape as cop-service /api/cop/objects) ─────
    println!("\n=== JSON (first object) ===");
    let json = serde_json::to_string_pretty(&objects[0])
        .expect("serialisation cannot fail for well-formed BattlespaceObject");
    println!("{json}");

    // ── 5. CEP distance helper ────────────────────────────────────────────────
    println!("\n=== CEP metrics ===");
    for obj in &objects {
        if let Some(cep) = obj.cep_m() {
            println!(
                "  {:10}  CEP≈{:.1}m",
                obj.label.as_deref().unwrap_or(&obj.id),
                cep,
            );
        }
    }

    println!("\nDone.");
}
