//! Demonstrates the `ExportAdapter` trait — exporting entities in JSON format.
//!
//! Export adapters convert internal Furia entities into standardised
//! formats (CoT XML, JC3IEDM, KML, JSON, etc.) for interoperability
//! with external systems like ATAK, NATO C2, and reporting tools.

use furia_sdk::export::{ExportAdapter, ExportEntity, ExportError, ExportFormat};
use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use uuid::Uuid;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;
const DEMO_DURATION_SECS: u64 = 3600;

/// An export adapter that serialises entities to pretty-printed JSON.
struct JsonExportAdapter;

impl ExportAdapter for JsonExportAdapter {
    fn export(&self, entity: &ExportEntity, _format: &ExportFormat, _handle: &ModuleHandle) -> Result<Vec<u8>, ExportError> {
        let json = serde_json::to_string_pretty(entity).map_err(|e| ExportError::Failed(e.to_string()))?;
        Ok(json.into_bytes())
    }

    fn supported_formats(&self) -> Vec<ExportFormat> {
        vec![ExportFormat::Custom("json".into())]
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let exporter = JsonExportAdapter;
    let handle = ModuleHandle::new_test(Uuid::new_v4());

    let entity = ExportEntity {
        entity_type: "track".into(),
        entity_id: "threat-t72-001".into(),
        attributes: serde_json::json!({
            "latitude": DEMO_LAT,
            "longitude": DEMO_LON,
            "altitude": 0.0,
            "classification": "hostile",
            "confidence": 0.95,
            "sensor": "radar"
        }),
    };

    println!("=== Export ===");
    let json = exporter.export(&entity, &ExportFormat::Custom("json".into()), &handle).expect("export should succeed for valid entity");
    println!("{}", String::from_utf8_lossy(&json));

    println!("\nSupported formats: {:?}", exporter.supported_formats());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_returns_non_empty() {
        let e = JsonExportAdapter;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let entity = ExportEntity { entity_type: "test".into(), entity_id: "id-1".into(), attributes: serde_json::json!({"key": "value"}) };
        let bytes = e.export(&entity, &ExportFormat::Custom("json".into()), &h).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_export_valid_json() {
        let e = JsonExportAdapter;
        let h = ModuleHandle::new_test(Uuid::new_v4());
        let entity = ExportEntity { entity_type: "test".into(), entity_id: "id-1".into(), attributes: serde_json::json!({"a": 1}) };
        let bytes = e.export(&entity, &ExportFormat::Custom("json".into()), &h).unwrap();
        let val: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(val["entity_type"], "test");
    }

    #[test]
    fn test_supported_formats_includes_json() {
        let e = JsonExportAdapter;
        assert!(e.supported_formats().contains(&ExportFormat::Custom("json".into())));
    }
}