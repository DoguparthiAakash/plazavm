//! Runtime images registry (R2: separate from templates).

use plaza_foundation::core::types::{Architecture, OperatingSystem};
use serde::{Deserialize, Serialize};

/// An available runtime image (e.g. "Ubuntu 24.04", "CUDA 12 Runtime").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeImage {
    pub id: String,
    pub name: String,
    pub description: String,
    pub os: OperatingSystem,
    pub arch: Vec<Architecture>,
    pub compatible_backends: Vec<String>,
}

pub struct RuntimeImageRegistry {
    images: Vec<RuntimeImage>,
}

impl RuntimeImageRegistry {
    pub fn new() -> Self {
        let default_images = vec![
            RuntimeImage {
                id: "ubuntu-24.04".into(),
                name: "Ubuntu 24.04 LTS".into(),
                description: "Standard Ubuntu Linux development environment".into(),
                os: OperatingSystem::Linux,
                arch: vec![Architecture::X86_64, Architecture::Aarch64],
                compatible_backends: vec!["docker".into(), "qemu".into(), "virtualbox".into()],
            },
            RuntimeImage {
                id: "cuda-12".into(),
                name: "CUDA 12 AI Runtime".into(),
                description: "NVIDIA CUDA 12 development runtime with PyTorch & TensorFlow".into(),
                os: OperatingSystem::Linux,
                arch: vec![Architecture::X86_64],
                compatible_backends: vec!["docker".into()],
            },
        ];

        Self {
            images: default_images,
        }
    }

    pub fn list(&self) -> &[RuntimeImage] {
        &self.images
    }

    pub fn get(&self, id: &str) -> Option<&RuntimeImage> {
        self.images.iter().find(|i| i.id == id)
    }
}

impl Default for RuntimeImageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

