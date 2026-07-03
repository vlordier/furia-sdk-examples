//! Demonstrates the `UiPlugin` trait — providing metadata and rendering
//! for a custom Furia panel.
//!
//! UI plugins extend the Furia interface with custom panels rendered
//! in iframes. The trait returns metadata (name, dimensions, preferred
//! slot) and a render output (path to an HTML asset) given current props.

use furia_sdk::module_handle::{ModuleHandle, ModuleHealth};
use furia_sdk::ui::{UiPlugin, UiPluginMetadata, UiPluginOutput, UiPluginProps};

/// A custom panel that shows an asset status summary.
struct AssetStatusPanel;

impl UiPlugin for AssetStatusPanel {
    fn render(&self, props: &UiPluginProps, _handle: &ModuleHandle) -> UiPluginOutput {
        let asset_info = props.selected_entity.as_ref()
            .and_then(|e| e.get("entity_id").and_then(|id| id.as_str()))
            .unwrap_or("none");
        UiPluginOutput {
            html_asset_path: format!("/plugins/asset-status/index.html?selected={}", asset_info),
        }
    }

    fn metadata(&self) -> UiPluginMetadata {
        UiPluginMetadata {
            name: "Asset Status Panel".into(),
            description: "Shows health, fuel, and ammo for the selected entity".into(),
            min_dimensions: (300, 200),
            preferred_slot: "right-sidebar".into(),
        }
    }

    fn health(&self) -> ModuleHealth { ModuleHealth::Healthy }
}

fn main() {
    let plugin = AssetStatusPanel;

    println!("=== UI Plugin ===");
    let meta = plugin.metadata();
    println!(" Name: {}", meta.name);
    println!(" Description: {}", meta.description);
    println!(" Slot: {} (min: {}x{})", meta.preferred_slot, meta.min_dimensions.0, meta.min_dimensions.1);

    let props = UiPluginProps {
        map_viewport: serde_json::json!({"zoom": 12, "center": [48.85, 2.35]}),
        selected_entity: Some(serde_json::json!({"entity_id": "drone-001", "type": "UAV"})),
        operator_role: "pilot".into(),
        screen_dimensions: (1920, 1080),
    };
    let output = plugin.render(&props, &furia_sdk::module_handle::ModuleHandle::new_test(uuid::Uuid::new_v4()));
    println!(" Rendered: {}", output.html_asset_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_metadata_name() {
        let p = AssetStatusPanel;
        assert_eq!(p.metadata().name, "Asset Status Panel");
    }

    #[test]
    fn test_render_with_selected_entity() {
        let p = AssetStatusPanel;
        let props = UiPluginProps {
            map_viewport: serde_json::json!({}), selected_entity: Some(serde_json::json!({"entity_id": "uav-1"})),
            operator_role: "pilot".into(), screen_dimensions: (1920, 1080),
        };
        let out = p.render(&props, &ModuleHandle::new_test(Uuid::new_v4()));
        assert!(out.html_asset_path.contains("uav-1"));
    }

    #[test]
    fn test_render_without_selected_entity() {
        let p = AssetStatusPanel;
        let props = UiPluginProps {
            map_viewport: serde_json::json!({}), selected_entity: None,
            operator_role: "pilot".into(), screen_dimensions: (1920, 1080),
        };
        let out = p.render(&props, &ModuleHandle::new_test(Uuid::new_v4()));
        assert!(out.html_asset_path.contains("none"));
    }
}