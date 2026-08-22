# arcbox-ext4 Patches

**Upstream Version:** `0.1.2`

## Reason for vendoring
`arcbox-ext4` handles ext4 filesystem formatting entirely in userspace without C-dependencies. However, version `0.1.2` contains a critical cross-platform bug in its directory path traversal (`src/file_tree.rs`) that causes it to unconditionally panic with `NotFound("/")` on Windows when provided an absolute Unix path.

PlazaVM must run natively on Windows, meaning the default implementation cannot be used.

## Patch Details
**Issue:** `FileTree::lookup()` uses `std::path::PathBuf::components()` to split Unix paths. On Windows, a leading slash (`/`) is parsed as a `std::path::Component::RootDir`, which converts its inner string representation to `\` instead of `/`. `arcbox-ext4` attempts to look up a directory named `\`, which does not exist in its in-memory tree (which stored `/`).

**Fix:** Added checks in `src/file_tree.rs` to ignore `Component::RootDir` and `Component::Prefix` when iterating over components. This strips out the Windows-specific root path anomalies and ensures consistent cross-platform unix-like path matching.

```rust
// diff
+            let is_root = component == std::path::Component::RootDir;
+            if is_root {
+                continue;
+            }
+            if let std::path::Component::Prefix(_) = component {
+                continue;
+            }
```

The patch is applied directly to `src/file_tree.rs` inside this `third_party` directory.
