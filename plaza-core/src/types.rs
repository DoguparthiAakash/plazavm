//! Shared domain types used across all PlazaVM crates.

use serde::{Deserialize, Serialize};

/// CPU architecture of a workspace or host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    X86_64,
    Aarch64,
    Riscv64,
    Arm32,
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X86_64 => write!(f, "x86_64"),
            Self::Aarch64 => write!(f, "aarch64"),
            Self::Riscv64 => write!(f, "riscv64"),
            Self::Arm32 => write!(f, "arm32"),
        }
    }
}

/// Operating system family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperatingSystem {
    Linux,
    Windows,
    MacOS,
    FreeBSD,
    Android,
    Custom(String),
}

impl std::fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Linux => write!(f, "linux"),
            Self::Windows => write!(f, "windows"),
            Self::MacOS => write!(f, "macos"),
            Self::FreeBSD => write!(f, "freebsd"),
            Self::Android => write!(f, "android"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}

/// Health status for workspaces, plugins, and runtimes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// Everything is functioning normally.
    Healthy,
    /// Some issues detected but still operational.
    Degraded,
    /// Not functioning correctly.
    Unhealthy,
    /// Health state is unknown (not yet checked).
    #[default]
    Unknown,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Degraded => write!(f, "degraded"),
            Self::Unhealthy => write!(f, "unhealthy"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// UTC timestamp wrapper with serde support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp(pub chrono::DateTime<chrono::Utc>);

impl Timestamp {
    /// Current UTC time.
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }

    /// Parse from RFC 3339 string.
    pub fn parse(s: &str) -> Result<Self, chrono::ParseError> {
        Ok(Self(
            chrono::DateTime::parse_from_rfc3339(s)?.with_timezone(&chrono::Utc),
        ))
    }

    /// Format as RFC 3339 string.
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

/// Resource size specification with human-friendly parsing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ByteSize(pub u64);

impl ByteSize {
    pub const fn from_mb(mb: u64) -> Self {
        Self(mb * 1024 * 1024)
    }

    pub const fn from_gb(gb: u64) -> Self {
        Self(gb * 1024 * 1024 * 1024)
    }

    pub fn as_mb(&self) -> u64 {
        self.0 / (1024 * 1024)
    }

    pub fn as_gb(&self) -> u64 {
        self.0 / (1024 * 1024 * 1024)
    }

    /// Parse human-readable sizes like "4Gi", "512Mi", "100G".
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if let Some(n) = s.strip_suffix("Gi") {
            let v: u64 = n.trim().parse().map_err(|e| format!("invalid size: {e}"))?;
            Ok(Self::from_gb(v))
        } else if let Some(n) = s.strip_suffix("Mi") {
            let v: u64 = n.trim().parse().map_err(|e| format!("invalid size: {e}"))?;
            Ok(Self::from_mb(v))
        } else if let Some(n) = s.strip_suffix('G') {
            let v: u64 = n.trim().parse().map_err(|e| format!("invalid size: {e}"))?;
            Ok(Self::from_gb(v))
        } else if let Some(n) = s.strip_suffix('M') {
            let v: u64 = n.trim().parse().map_err(|e| format!("invalid size: {e}"))?;
            Ok(Self::from_mb(v))
        } else {
            let v: u64 = s.parse().map_err(|e| format!("invalid size: {e}"))?;
            Ok(Self(v))
        }
    }
}

impl std::fmt::Display for ByteSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 >= 1024 * 1024 * 1024 && self.0.is_multiple_of(1024 * 1024 * 1024) {
            write!(f, "{}Gi", self.as_gb())
        } else {
            write!(f, "{}Mi", self.as_mb())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_size_parsing() {
        assert_eq!(ByteSize::parse("4Gi").unwrap().as_gb(), 4);
        assert_eq!(ByteSize::parse("512Mi").unwrap().as_mb(), 512);
        assert_eq!(ByteSize::parse("2G").unwrap().as_gb(), 2);
    }

    #[test]
    fn timestamp_roundtrip() {
        let ts = Timestamp::now();
        let s = ts.to_rfc3339();
        let parsed = Timestamp::parse(&s).unwrap();
        assert_eq!(ts.0.timestamp(), parsed.0.timestamp());
    }

    #[test]
    fn health_status_default_is_unknown() {
        assert_eq!(HealthStatus::default(), HealthStatus::Unknown);
    }
}
