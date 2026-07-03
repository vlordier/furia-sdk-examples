//! Hello NATO/France Coalition example.
//! Demonstrates consistent use of shared types from furia-core (Nation, Marking, Caveat)
//! across SDK consumers, control, and examples.

use durandal_types::nato::{Nation, MarkingProfile, NationalCaveat, passes_marking, getOriginShort};

fn main() {
    println!("Furia NATO Coalition Hello (consistent across repos)");

    let fr_track = ("FR-UAV-01", Some(Nation::FR), Some("ASTERIX"));
    let nato_track = ("NATO-EX-42", Some(Nation::NATO), Some("C3"));

    println!("Origin: {}", getOriginShort(&fr_track)); // would be FR·AST in UI
    println!("Origin: {}", getOriginShort(&nato_track));

    let session = MarkingProfile::REL_FR_UA;
    let payload = MarkingProfile::NATO_RESTRICTED;
    println!("Passes marking? {}", passes_marking(&session, &payload));

    let caveat = NationalCaveat {
        drone_type: "Eurodrone".into(),
        max_autonomy_level: 4,
        max_lethal_level: 4,
        operator_nationalities: vec![Nation::FR, Nation::DE, Nation::IT, Nation::ES],
        source: "OCCAR".into(),
    };
    println!("Caveat for {} allows operators: {:?}", caveat.drone_type, caveat.operator_nationalities);

    println!("Use with furia-core SDK + durandal-furia-control for full multi-nation C2.");
}