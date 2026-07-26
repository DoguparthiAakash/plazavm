//! Evidence-Driven QA Certification Framework for PlazaVM v2.

pub mod dashboard;
pub mod evidence;
pub mod reporter;
pub mod runner;
pub mod stages;

pub use evidence::{CommandResult, EvidenceCollector, StageEvidence};
pub use runner::ValidationPipeline;
use serde::{Deserialize, Serialize};

/// Structured QA Certification Telemetry Data holding results and evidence of all 16 Stages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRunReport {
    pub timestamp: String,
    pub git_commit: String,
    pub rust_version: String,
    pub system_info: SystemValidationInfo,
    pub stages: Vec<StageResult>,
    pub total_commands_executed: usize,
    pub total_artifacts_generated: usize,
    pub evidence_completeness_pct: f64,
    pub overall_health_score: u32,
    pub overall_grade: String,
    pub production_ready: bool,
    pub quality_gates_passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemValidationInfo {
    pub os: String,
    pub arch: String,
    pub logical_cores: u32,
    pub memory_total_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageResult {
    pub stage_number: u32,
    pub name: String,
    pub status: StageStatus,
    pub duration_ms: u128,
    pub summary: String,
    pub details: Vec<String>,
    pub log_file: String,
    pub metrics_file: Option<String>,
    pub commands: Vec<CommandResult>,
    pub artifacts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StageStatus {
    Passed,
    Failed,
    Skipped,
}

