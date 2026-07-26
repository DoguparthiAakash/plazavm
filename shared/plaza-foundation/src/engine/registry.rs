use crate::engine::errors::{PfeError, PfeResult};
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

pub struct ServiceNode {
    pub name: String,
    pub dependencies: Vec<String>,
    pub initialized: bool,
}

/// Service Registry with cycle detection and dependency-ordered initialization.
pub struct ServiceRegistry {
    nodes: RwLock<HashMap<String, ServiceNode>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            nodes: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, name: impl Into<String>, dependencies: Vec<String>) -> PfeResult<()> {
        let name_str = name.into();
        let mut nodes = self.nodes.write().unwrap();

        nodes.insert(
            name_str.clone(),
            ServiceNode {
                name: name_str,
                dependencies,
                initialized: false,
            },
        );
        Ok(())
    }

    pub fn validate_cycles(&self) -> PfeResult<()> {
        let nodes = self.nodes.read().unwrap();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for name in nodes.keys() {
            if self.detect_cycle(name, &nodes, &mut visited, &mut rec_stack) {
                return Err(PfeError::CyclicDependency(name.clone()));
            }
        }
        Ok(())
    }

    fn detect_cycle(
        &self,
        node: &str,
        nodes: &HashMap<String, ServiceNode>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        if !visited.contains(node) {
            visited.insert(node.to_string());
            rec_stack.insert(node.to_string());

            if let Some(service) = nodes.get(node) {
                for dep in &service.dependencies {
                    if (!visited.contains(dep) && self.detect_cycle(dep, nodes, visited, rec_stack))
                        || rec_stack.contains(dep)
                    {
                        return true;
                    }
                }
            }
        }
        rec_stack.remove(node);
        false
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_service_registry() {
        let registry = ServiceRegistry::new();
        registry.register("storage", vec![]).unwrap();
        registry
            .register("workspace", vec!["storage".into()])
            .unwrap();
        assert!(registry.validate_cycles().is_ok());
    }

    #[test]
    fn test_cyclic_service_registry() {
        let registry = ServiceRegistry::new();
        registry
            .register("service_a", vec!["service_b".into()])
            .unwrap();
        registry
            .register("service_b", vec!["service_a".into()])
            .unwrap();
        assert!(registry.validate_cycles().is_err());
    }
}
