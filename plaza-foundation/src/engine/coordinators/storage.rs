use plaza_storage::SqliteWorkspaceRepository;
use std::sync::Arc;

/// Thin coordinator delegating storage operations to `plaza-storage`.
pub struct StorageCoordinator {
    repo: Arc<SqliteWorkspaceRepository>,
}

impl StorageCoordinator {
    pub fn new(repo: Arc<SqliteWorkspaceRepository>) -> Self {
        Self { repo }
    }

    pub fn repository(&self) -> &Arc<SqliteWorkspaceRepository> {
        &self.repo
    }
}
