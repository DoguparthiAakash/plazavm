//! Capability Resolver & Database for Workspace specifications.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDescriptor {
    pub name: String,
    pub version: semver::Version,
    pub dependencies: Vec<String>,
}

pub struct CapabilityDatabase {
    capabilities: HashMap<String, CapabilityDescriptor>,
}

impl Default for CapabilityDatabase {
    fn default() -> Self {
        let mut db = HashMap::new();
        db.insert(
            "python".into(),
            CapabilityDescriptor {
                name: "python".into(),
                version: semver::Version::new(3, 12, 0),
                dependencies: vec!["c-compiler".into()],
            },
        );
        db.insert(
            "node".into(),
            CapabilityDescriptor {
                name: "node".into(),
                version: semver::Version::new(20, 0, 0),
                dependencies: Vec::new(),
            },
        );
        Self { capabilities: db }
    }
}

pub struct CapabilityResolver {
    db: CapabilityDatabase,
}

impl Default for CapabilityResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityResolver {
    pub fn new() -> Self {
        Self {
            db: CapabilityDatabase::default(),
        }
    }

    pub fn resolve(&self, reqs: &[String]) -> Vec<CapabilityDescriptor> {
        let mut resolved = Vec::new();
        for req in reqs {
            if let Some(desc) = self.db.capabilities.get(req) {
                resolved.push(desc.clone());
            }
        }
        resolved
    }
}
