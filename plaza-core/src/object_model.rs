//! Plaza Object Model (POM) header and metadata models.

use crate::puri::PlazaUri;
use crate::types::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard Plaza Object Header for all platform resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlazaObjectHeader {
    pub id: String,
    pub puri: PlazaUri,
    pub name: String,
    pub version: semver::Version,
    pub labels: HashMap<String, String>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub owner: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub status: String,
    pub lifecycle_state: String,
}

impl PlazaObjectHeader {
    pub fn new(
        id: impl Into<String>,
        namespace: &str,
        name: impl Into<String>,
        version: semver::Version,
    ) -> Self {
        let name_str = name.into();
        let id_str = id.into();
        let puri = PlazaUri::new(namespace, &id_str);

        Self {
            id: id_str,
            puri,
            name: name_str,
            version,
            labels: HashMap::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
            owner: "system".into(),
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            status: "active".into(),
            lifecycle_state: "ready".into(),
        }
    }
}
