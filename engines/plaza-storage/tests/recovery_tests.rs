use plaza_foundation::core::id::WorkspaceId;
use plaza_storage::SqliteWorkspaceRepository;

#[test]
fn test_sqlite_restart_persistence_and_recovery() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let db_path = temp_dir.path().join("test_recovery.db");

    let id = WorkspaceId::new();

    // 1. Open DB, save workspace record
    {
        let repo = SqliteWorkspaceRepository::open(db_path.clone()).expect("open db");
        repo.save_raw(&id, "persistent-ws", Some("desc"), "{}", "{}", "{}")
            .expect("save raw record");
    }

    // 2. Simulate process crash/shutdown & re-open connection
    {
        let repo_recovered = SqliteWorkspaceRepository::open(db_path).expect("re-open db");
        let fetched = repo_recovered
            .get_raw(&id)
            .expect("fetch should succeed")
            .expect("workspace record must exist after crash restart");

        assert_eq!(fetched.0, "persistent-ws");
        assert_eq!(fetched.1.as_deref(), Some("desc"));
    }
}

