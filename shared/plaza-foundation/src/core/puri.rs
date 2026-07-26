//! Plaza Universal Resource Identifier (PURI) parser and validator.
//!
//! Syntax: `plaza://<namespace>/<resource_id>[?query][#fragment]`

use crate::core::error::PlazaError;
use crate::core::PlazaResult;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Plaza Universal Resource Identifier (PURI).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlazaUri {
    pub namespace: String,
    pub resource_id: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl PlazaUri {
    pub fn new(namespace: impl Into<String>, resource_id: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            resource_id: resource_id.into(),
            query: None,
            fragment: None,
        }
    }

    pub fn parse(s: &str) -> PlazaResult<Self> {
        if !s.starts_with("plaza://") {
            return Err(PlazaError::config(format!(
                "Invalid PURI format '{}': must start with 'plaza://'",
                s
            )));
        }

        let rest = &s["plaza://".len()..];
        let mut parts = rest.splitn(2, '/');
        let ns = parts.next().unwrap_or("");
        let path_rest = parts.next().unwrap_or("");

        if ns.is_empty() || path_rest.is_empty() {
            return Err(PlazaError::config(format!(
                "Invalid PURI format '{}': expected plaza://<namespace>/<resource_id>",
                s
            )));
        }

        let (resource_id, query, fragment) = parse_path_rest(path_rest);

        Ok(Self {
            namespace: ns.to_string(),
            resource_id,
            query,
            fragment,
        })
    }
}

fn parse_path_rest(rest: &str) -> (String, Option<String>, Option<String>) {
    let mut fragment = None;
    let mut remaining = rest;

    if let Some(pos) = remaining.find('#') {
        fragment = Some(remaining[pos + 1..].to_string());
        remaining = &remaining[..pos];
    }

    let mut query = None;
    if let Some(pos) = remaining.find('?') {
        query = Some(remaining[pos + 1..].to_string());
        remaining = &remaining[..pos];
    }

    (remaining.to_string(), query, fragment)
}

impl fmt::Display for PlazaUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "plaza://{}/{}", self.namespace, self.resource_id)?;
        if let Some(ref q) = self.query {
            write!(f, "?{}", q)?;
        }
        if let Some(ref frag) = self.fragment {
            write!(f, "#{}", frag)?;
        }
        Ok(())
    }
}

impl FromStr for PlazaUri {
    type Err = PlazaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puri_parsing() {
        let uri = PlazaUri::parse("plaza://workspace/my-ai-lab?env=prod#main").unwrap();
        assert_eq!(uri.namespace, "workspace");
        assert_eq!(uri.resource_id, "my-ai-lab");
        assert_eq!(uri.query.as_deref(), Some("env=prod"));
        assert_eq!(uri.fragment.as_deref(), Some("main"));
        assert_eq!(uri.to_string(), "plaza://workspace/my-ai-lab?env=prod#main");
    }
}


