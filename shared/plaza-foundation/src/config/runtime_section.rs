//! Runtime section of `plaza.yaml`.
//!
//! Controls backend selection and acceleration policy.
//! Does NOT describe the machine hardware — that belongs in `machine_section`.

use serde::{Deserialize, Serialize};

/// The `runtime:` section of `plaza.yaml`.
///
/// # Example
///
/// ```yaml
/// runtime:
///   backend: auto
///   acceleration:
///     enabled: false
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSection {
    /// Backend selection: `"auto"`, `"qemu"`, `"v86"`.
    #[serde(default = "default_backend")]
    pub backend: String,

    /// Acceleration policy.
    #[serde(default)]
    pub acceleration: AccelerationSection,
}

fn default_backend() -> String {
    "auto".into()
}

impl Default for RuntimeSection {
    fn default() -> Self {
        Self {
            backend: "auto".into(),
            acceleration: AccelerationSection::default(),
        }
    }
}

/// Acceleration policy.
///
/// PlazaVM's foundational engine uses software emulation only.
/// Hardware acceleration (KVM, WHPX, HVF) is NOT enabled by default
/// and must be explicitly requested.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelerationSection {
    /// Whether hardware acceleration is enabled.
    /// Default: `false` — TCG/software emulation only.
    #[serde(default)]
    pub enabled: bool,
}

impl Default for AccelerationSection {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// Parsed and validated backend preference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendPreference {
    /// Let the runtime resolver choose.
    Auto,
    /// Explicitly request QEMU-TCG.
    QemuTcg,
    /// Explicitly request v86-WASM.
    V86Wasm,
}

impl RuntimeSection {
    /// Parse the `backend` string into a typed preference.
    pub fn parsed_backend(&self) -> Result<BackendPreference, String> {
        match self.backend.to_lowercase().as_str() {
            "auto" => Ok(BackendPreference::Auto),
            "qemu" | "qemu-tcg" => Ok(BackendPreference::QemuTcg),
            "v86" | "v86-wasm" => Ok(BackendPreference::V86Wasm),
            other => Err(format!("unknown runtime backend: '{}'", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let section = RuntimeSection::default();
        assert_eq!(section.backend, "auto");
        assert!(!section.acceleration.enabled);
    }

    #[test]
    fn parse_backend_auto() {
        let s = RuntimeSection {
            backend: "auto".into(),
            ..Default::default()
        };
        assert_eq!(s.parsed_backend().unwrap(), BackendPreference::Auto);
    }

    #[test]
    fn parse_backend_qemu() {
        let s = RuntimeSection {
            backend: "qemu".into(),
            ..Default::default()
        };
        assert_eq!(s.parsed_backend().unwrap(), BackendPreference::QemuTcg);
    }

    #[test]
    fn parse_backend_v86() {
        let s = RuntimeSection {
            backend: "v86".into(),
            ..Default::default()
        };
        assert_eq!(s.parsed_backend().unwrap(), BackendPreference::V86Wasm);
    }

    #[test]
    fn parse_backend_invalid() {
        let s = RuntimeSection {
            backend: "hyperv".into(),
            ..Default::default()
        };
        assert!(s.parsed_backend().is_err());
    }

    #[test]
    fn acceleration_disabled_by_default() {
        let yaml = "backend: auto\n";
        let section: RuntimeSection = serde_yaml::from_str(yaml).unwrap();
        assert!(!section.acceleration.enabled);
    }
}
