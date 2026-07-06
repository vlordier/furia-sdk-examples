//! Demonstrates the `DecisionTreeProvider` trait — training and evaluating
//! a simple threshold-based classifier.
//!
//! Decision tree modules compile labelled training examples into a tree
//! structure, then evaluate new feature vectors to produce classifications.

use std::collections::HashMap;

use furia_sdk::decision_tree::{DecisionTree, DecisionTreeProvider, TrainingExample};
use serde_json::Value;

// ── Demo constants ──────────────────────────────────────────────
const DEMO_LAT: f64 = 48.85;
const DEMO_LON: f64 = 2.35;
const DEMO_DURATION_SECS: u64 = 3600;

/// A single-threshold classifier trained on labelled examples.
struct ThresholdTree {
    threshold: f32,
    pos_label: String,
    neg_label: String,
}

impl ThresholdTree {
    fn new(threshold: f32, pos_label: &str, neg_label: &str) -> Self {
        Self { threshold, pos_label: pos_label.into(), neg_label: neg_label.into() }
    }
}

impl DecisionTreeProvider for ThresholdTree {
    fn train(&self, examples: &[TrainingExample]) -> Result<DecisionTree, String> {
        if examples.is_empty() {
            return Err("no training examples".into());
        }
        // Simple mean of the first feature across all examples
        let sum: f32 = examples.iter().filter_map(|ex| ex.features.values().next()).sum();
        let threshold = sum / examples.len() as f32;
        let majority = if examples.iter().filter(|ex| ex.label == self.pos_label).count()
            > examples.len() / 2 { &self.pos_label } else { &self.neg_label };
        Ok(DecisionTree {
            inner: serde_json::json!({
                "type": "threshold",
                "threshold": threshold,
                "default": majority,
                "n_examples": examples.len(),
            }),
        })
    }

    fn evaluate(&self, tree: &DecisionTree, features: &HashMap<String, f32>) -> Result<String, String> {
        let threshold = tree.inner["threshold"].as_f64().unwrap_or(0.5) as f32;
        let default = tree.inner["default"].as_str().unwrap_or("unknown");
        match features.values().next() {
            Some(&val) if val >= threshold => Ok(self.pos_label.clone()),
            _ => Ok(default.to_string()),
        }
    }

    fn metadata(&self) -> Value {
        serde_json::json!({"provider": "ThresholdTree", "threshold": self.threshold, "version": "1.0"})
    }
}

fn main() {
    let tree = ThresholdTree::new(0.5, "threat", "benign");

    let examples = vec![
        TrainingExample { features: HashMap::from([("rssi".into(), 0.9)]), label: "threat".into() },
        TrainingExample { features: HashMap::from([("rssi".into(), 0.1)]), label: "benign".into() },
    ];

    println!("=== Decision Tree ===");
    let trained = tree.train(&examples).expect("training with valid examples should succeed");
    println!(" Trained tree: {}", serde_json::to_string_pretty(&trained.inner).expect("trained tree should serialize to JSON"));

    let high = tree.evaluate(&trained, &HashMap::from([("rssi".into(), 0.85)])).expect("evaluation of high RSSI should succeed");
    println!(" High RSSI -> {}", high);

    let low = tree.evaluate(&trained, &HashMap::from([("rssi".into(), 0.05)])).expect("evaluation of low RSSI should succeed");
    println!(" Low RSSI  -> {}", low);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train_returns_tree() {
        let t = ThresholdTree::new(0.5, "A", "B");
        let tree = t.train(&[TrainingExample { features: HashMap::from([("x".into(), 1.0)]), label: "A".into() }]).unwrap();
        assert_eq!(tree.inner["type"], "threshold");
    }

    #[test]
    fn test_evaluate_above_threshold() {
        let t = ThresholdTree::new(0.5, "threat", "benign");
        let tree = DecisionTree { inner: serde_json::json!({"type": "threshold", "threshold": 0.5, "default": "benign"}) };
        let r = t.evaluate(&tree, &HashMap::from([("rssi".into(), 0.9)])).unwrap();
        assert_eq!(r, "threat");
    }

    #[test]
    fn test_metadata_contains_provider() {
        let t = ThresholdTree::new(0.5, "A", "B");
        assert_eq!(t.metadata()["provider"], "ThresholdTree");
    }
}