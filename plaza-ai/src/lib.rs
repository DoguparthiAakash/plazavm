//! # plaza-ai
//!
//! AI Advisor System.
//!
//! AI never mutates core state directly — it acts strictly as an advisory system,
//! providing runtime recommendations, resource optimization advice, error explanations,
//! and project structure analysis.

pub mod advisor;
pub mod provider;

pub use advisor::AiAdvisorSystem;
pub use provider::{AiProvider, OllamaProvider, OpenAIProvider};
