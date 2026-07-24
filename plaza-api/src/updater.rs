//! Automatic Update Framework Abstraction.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateChannel {
    Stable,
    Beta,
    DevPreview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionCheckResult {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub channel: UpdateChannel,
    pub release_notes: String,
}

pub struct UpdateService;

impl UpdateService {
    pub async fn check_for_updates(channel: UpdateChannel) -> anyhow::Result<VersionCheckResult> {
        let current_version = env!("CARGO_PKG_VERSION").to_string();

        let latest_version = "0.1.0-dp1".to_string();
        let update_available = semver::Version::parse(&latest_version)
            .ok()
            .zip(semver::Version::parse(&current_version).ok())
            .is_some_and(|(latest, current)| latest > current);

        Ok(VersionCheckResult {
            current_version,
            latest_version,
            update_available,
            channel,
            release_notes: "PlazaVM v0.1.0-dp1 Developer Preview 1 release with desktop shell, diagnostic bundles, and validation pipeline.".into(),
        })
    }
}
