# `plaza-storage`

Persistence layer crate providing thread-safe SQLite database repositories.

---

## 🛠 Responsibilities

- Thread-safe SQLite repository (`SqliteWorkspaceRepository`).
- In-memory database mode for zero-disk integration tests.
- Parameterized `rusqlite` SQL queries guaranteeing 100% SQL injection immunity.
