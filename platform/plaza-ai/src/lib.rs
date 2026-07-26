//! # plaza-ai
//!
//! AI Advisor System.
//!
//! AI never mutates core state directly - it acts strictly as an advisory system,
//! providing runtime recommendations, resource optimization advice, error explanations,
//! and project structure analysis.

pub mod inference;
pub mod context;
pub mod codegen;
pub mod troubleshoot;
pub mod vector_db;
pub mod advisor;
pub mod provider;

pub use advisor::AiAdvisorSystem;
pub use provider::{AiProvider, OllamaProvider, OpenAIProvider};

