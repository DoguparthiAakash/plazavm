//! Comprehensive tests for the PlazaVM Image Engine (Phase 11).

#[cfg(test)]
mod blob_store_tests {
    use plaza_image::store::blob::{BlobStore, LocalBlobStore};
    use plaza_image::model::ContentHash;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_put_and_get() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalBlobStore::new(dir.path().join("blobs")).await.unwrap();

        let data = b"hello plazavm image engine";
        let mut reader = Cursor::new(data.as_ref());
        let hash = store.put_stream(&mut reader).await.unwrap();

        assert!(store.exists(&hash).await.unwrap());

        let path = store.get_path(&hash).unwrap();
        let read_back = tokio::fs::read(&path).await.unwrap();
        assert_eq!(read_back, data);
    }

    #[tokio::test]
    async fn test_put_duplicate_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalBlobStore::new(dir.path().join("blobs")).await.unwrap();

        let data = b"duplicate content";
        let mut r1 = Cursor::new(data.as_ref());
        let h1 = store.put_stream(&mut r1).await.unwrap();

        let mut r2 = Cursor::new(data.as_ref());
        let h2 = store.put_stream(&mut r2).await.unwrap();

        assert_eq!(h1, h2);
    }

    #[tokio::test]
    async fn test_remove() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalBlobStore::new(dir.path().join("blobs")).await.unwrap();

        let data = b"remove me";
        let mut reader = Cursor::new(data.as_ref());
        let hash = store.put_stream(&mut reader).await.unwrap();
        assert!(store.exists(&hash).await.unwrap());

        store.remove(&hash).await.unwrap();
        assert!(!store.exists(&hash).await.unwrap());
    }

    #[tokio::test]
    async fn test_exists_false_for_missing() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalBlobStore::new(dir.path().join("blobs")).await.unwrap();

        let fake_hash = ContentHash::new_sha256(
            "0000000000000000000000000000000000000000000000000000000000000000"
        ).unwrap();
        assert!(!store.exists(&fake_hash).await.unwrap());
    }

    #[tokio::test]
    async fn test_large_blob_streaming() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalBlobStore::new(dir.path().join("blobs")).await.unwrap();

        // 1 MB blob
        let data = vec![0xABu8; 1024 * 1024];
        let mut reader = Cursor::new(data.as_slice());
        let hash = store.put_stream(&mut reader).await.unwrap();

        assert!(store.exists(&hash).await.unwrap());
        let path = store.get_path(&hash).unwrap();
        let read_back = tokio::fs::read(&path).await.unwrap();
        assert_eq!(read_back.len(), 1024 * 1024);
    }
}

#[cfg(test)]
mod content_hash_tests {
    use plaza_image::model::{ContentHash, HashAlgorithm};
    use std::str::FromStr;

    #[test]
    fn test_valid_sha256() {
        let hash = ContentHash::new_sha256(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        ).unwrap();
        assert_eq!(hash.algorithm, HashAlgorithm::Sha256);
        assert_eq!(hash.to_string(), "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn test_invalid_digest_length() {
        assert!(ContentHash::new_sha256("abc123").is_err());
    }

    #[test]
    fn test_invalid_digest_chars() {
        assert!(ContentHash::new_sha256(
            "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz"
        ).is_err());
    }

    #[test]
    fn test_from_str_roundtrip() {
        let s = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let hash = ContentHash::from_str(s).unwrap();
        assert_eq!(hash.to_string(), s);
    }

    #[test]
    fn test_from_str_invalid_format() {
        assert!(ContentHash::from_str("invalid").is_err());
    }

    #[test]
    fn test_from_str_unsupported_algorithm() {
        assert!(ContentHash::from_str("md5:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789").is_err());
    }
}

#[cfg(test)]
mod manifest_store_tests {
    use plaza_image::store::manifest::{ManifestStore, LocalManifestStore};
    use plaza_image::model::*;
    use plaza_foundation::core::types::Timestamp;
    use std::collections::HashMap;

    fn sample_manifest() -> ImageManifest {
        ImageManifest {
            version: 1,
            name: "test-image".to_string(),
            image_version: "1.0".to_string(),
            architecture: "x86_64".to_string(),
            layers: vec![ImageLayer {
                digest: ContentHash::new_sha256(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ).unwrap(),
                size: 1024,
                media_type: LayerMediaType::RawBlock,
            }],
            kernel: None,
            initrd: None,
            boot: BootMetadata::default(),
            metadata: ImageMetadata {
                created_at: Timestamp::now(),
                author: Some("test".to_string()),
                labels: HashMap::new(),
            },
        }
    }

    #[tokio::test]
    async fn test_put_and_get_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalManifestStore::new(dir.path().join("manifests")).await.unwrap();

        let manifest = sample_manifest();
        store.put_manifest(&manifest).await.unwrap();

        let retrieved = store.get_manifest("test-image", "1.0").await.unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.name, "test-image");
        assert_eq!(retrieved.image_version, "1.0");
    }

    #[tokio::test]
    async fn test_get_missing_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalManifestStore::new(dir.path().join("manifests")).await.unwrap();

        let result = store.get_manifest("nonexistent", "1.0").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_remove_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let store = LocalManifestStore::new(dir.path().join("manifests")).await.unwrap();

        let manifest = sample_manifest();
        store.put_manifest(&manifest).await.unwrap();

        store.remove_manifest("test-image", "1.0").await.unwrap();
        let result = store.get_manifest("test-image", "1.0").await.unwrap();
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod resolver_tests {
    use plaza_image::resolver::parse_image_ref;

    #[test]
    fn test_parse_name_only() {
        let r = parse_image_ref("alpine").unwrap();
        assert_eq!(r.name, "alpine");
        assert_eq!(r.tag, Some("latest".to_string()));
        assert!(r.digest.is_none());
    }

    #[test]
    fn test_parse_name_tag() {
        let r = parse_image_ref("alpine:3.18").unwrap();
        assert_eq!(r.name, "alpine");
        assert_eq!(r.tag, Some("3.18".to_string()));
        assert!(r.digest.is_none());
    }

    #[test]
    fn test_parse_name_digest() {
        let r = parse_image_ref("alpine@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855").unwrap();
        assert_eq!(r.name, "alpine");
        assert!(r.tag.is_none());
        assert!(r.digest.is_some());
    }

    #[test]
    fn test_parse_empty_errors() {
        assert!(parse_image_ref("").is_err());
    }

    #[test]
    fn test_parse_invalid_digest_errors() {
        assert!(parse_image_ref("alpine@sha256:invalid").is_err());
    }
}

#[cfg(test)]
mod virtual_block_device_tests {
    use plaza_image::block::{VirtualBlockDevice, FileBackedImmutableLayer, CowWritableLayer, BLOCK_SIZE};
    use plaza_image::composer::LayeredBlockDevice;
    use plaza_image::block::ImmutableLayer;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_read_from_single_immutable_layer() {
        let dir = tempfile::tempdir().unwrap();
        let base_path = dir.path().join("base.raw");

        // Write a base image with known content
        let mut content = vec![0u8; 8192]; // 2 blocks
        content[0..5].copy_from_slice(b"hello");
        content[4096..4101].copy_from_slice(b"world");
        tokio::fs::write(&base_path, &content).await.unwrap();

        let layer = Arc::new(FileBackedImmutableLayer::open(base_path).await.unwrap());
        let cow_path = dir.path().join("cow.raw");
        let cow = CowWritableLayer::create(cow_path, 8192).await.unwrap();

        let mut device = LayeredBlockDevice::new(vec![layer], cow);
        assert_eq!(device.size(), 8192);

        // Read block 0
        let mut buf = vec![0u8; 5];
        let n = device.read_at(0, &mut buf).await.unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"hello");

        // Read block 1
        let mut buf2 = vec![0u8; 5];
        let n = device.read_at(4096, &mut buf2).await.unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf2, b"world");
    }

    #[tokio::test]
    async fn test_write_goes_to_cow_layer() {
        let dir = tempfile::tempdir().unwrap();
        let base_path = dir.path().join("base.raw");

        let content = vec![0xAAu8; 4096];
        tokio::fs::write(&base_path, &content).await.unwrap();

        let layer = Arc::new(FileBackedImmutableLayer::open(base_path.clone()).await.unwrap());
        let cow_path = dir.path().join("cow.raw");
        let cow = CowWritableLayer::create(cow_path, 4096).await.unwrap();

        let mut device = LayeredBlockDevice::new(vec![layer], cow);

        // Write new data
        device.write_at(0, b"MODIFIED").await.unwrap();

        // Read back — should get COW version
        let mut buf = vec![0u8; 8];
        let n = device.read_at(0, &mut buf).await.unwrap();
        assert_eq!(n, 8);
        assert_eq!(&buf, b"MODIFIED");

        // Verify base layer is unchanged
        let base_data = tokio::fs::read(&base_path).await.unwrap();
        assert_eq!(base_data[0], 0xAA);
    }

    #[tokio::test]
    async fn test_layer_override_priority() {
        let dir = tempfile::tempdir().unwrap();

        // Base layer: "AAAA"
        let base_path = dir.path().join("base.raw");
        tokio::fs::write(&base_path, vec![0xAAu8; 4096]).await.unwrap();

        // Override layer: "BBBB"
        let over_path = dir.path().join("override.raw");
        tokio::fs::write(&over_path, vec![0xBBu8; 4096]).await.unwrap();

        let base = Arc::new(FileBackedImmutableLayer::open(base_path).await.unwrap());
        let over = Arc::new(FileBackedImmutableLayer::open(over_path).await.unwrap());

        let cow_path = dir.path().join("cow.raw");
        let cow = CowWritableLayer::create(cow_path, 4096).await.unwrap();

        // Over is added after base, so it has higher priority
        let device = LayeredBlockDevice::new(vec![base, over], cow);

        let mut buf = vec![0u8; 1];
        device.read_at(0, &mut buf).await.unwrap();
        assert_eq!(buf[0], 0xBB, "Higher layer should override base");
    }

    #[tokio::test]
    async fn test_out_of_bounds_read() {
        let dir = tempfile::tempdir().unwrap();
        let base_path = dir.path().join("base.raw");
        tokio::fs::write(&base_path, vec![0u8; 4096]).await.unwrap();

        let layer = Arc::new(FileBackedImmutableLayer::open(base_path).await.unwrap());
        let cow_path = dir.path().join("cow.raw");
        let cow = CowWritableLayer::create(cow_path, 4096).await.unwrap();

        let device = LayeredBlockDevice::new(vec![layer], cow);

        let mut buf = vec![0u8; 1];
        let result = device.read_at(4096, &mut buf).await;
        assert!(result.is_err(), "Reading at exact size boundary should error");
    }

    #[tokio::test]
    async fn test_partial_write_preserves_unrelated_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let base_path = dir.path().join("base.raw");

        // Fill a block with known pattern
        let mut content = vec![0u8; 4096];
        for (i, byte) in content.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }
        tokio::fs::write(&base_path, &content).await.unwrap();

        let layer = Arc::new(FileBackedImmutableLayer::open(base_path).await.unwrap());
        let cow_path = dir.path().join("cow.raw");
        let cow = CowWritableLayer::create(cow_path, 4096).await.unwrap();

        let mut device = LayeredBlockDevice::new(vec![layer], cow);

        // Write 20 bytes at offset 100 (sub-block partial write)
        let write_data = [0xFF; 20];
        let n = device.write_at(100, &write_data).await.unwrap();
        assert_eq!(n, 20);

        // Read back at offset 100 — should be 0xFF
        let mut buf = vec![0u8; 20];
        device.read_at(100, &mut buf).await.unwrap();
        assert_eq!(buf, vec![0xFF; 20]);

        // Read byte at offset 0 — should be original content (0)
        let mut buf0 = vec![0u8; 1];
        device.read_at(0, &mut buf0).await.unwrap();
        assert_eq!(buf0[0], 0);

        // Read byte at offset 200 — should be original content (200)
        let mut buf200 = vec![0u8; 1];
        device.read_at(200, &mut buf200).await.unwrap();
        assert_eq!(buf200[0], 200);
    }

    #[tokio::test]
    async fn test_flush() {
        let dir = tempfile::tempdir().unwrap();
        let base_path = dir.path().join("base.raw");
        tokio::fs::write(&base_path, vec![0u8; 4096]).await.unwrap();

        let layer = Arc::new(FileBackedImmutableLayer::open(base_path).await.unwrap());
        let cow_path = dir.path().join("cow.raw");
        let cow = CowWritableLayer::create(cow_path, 4096).await.unwrap();

        let mut device = LayeredBlockDevice::new(vec![layer], cow);
        device.write_at(0, b"test").await.unwrap();
        // Flush should not panic
        device.flush().await.unwrap();
    }
}

#[cfg(test)]
mod gc_tests {
    use plaza_image::gc::{GarbageCollector, LocalGarbageCollector};
    use plaza_image::model::ContentHash;
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_gc_dry_run_does_not_delete() {
        let dir = tempfile::tempdir().unwrap();
        let blob_dir = dir.path().join("blobs");
        let sha_dir = blob_dir.join("sha256");
        tokio::fs::create_dir_all(&sha_dir).await.unwrap();

        // Create a fake blob
        let digest = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        tokio::fs::write(sha_dir.join(digest), b"data").await.unwrap();

        let gc = LocalGarbageCollector::new(&blob_dir);
        let reachable = HashSet::new();

        let report = gc.run_gc(&reachable, true).await.unwrap();
        assert_eq!(report.deleted_blobs, 1);

        // File should still exist because dry_run = true
        assert!(sha_dir.join(digest).exists());
    }

    #[tokio::test]
    async fn test_gc_deletes_unreachable() {
        let dir = tempfile::tempdir().unwrap();
        let blob_dir = dir.path().join("blobs");
        let sha_dir = blob_dir.join("sha256");
        tokio::fs::create_dir_all(&sha_dir).await.unwrap();

        let digest = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        tokio::fs::write(sha_dir.join(digest), b"data").await.unwrap();

        let gc = LocalGarbageCollector::new(&blob_dir);
        let reachable = HashSet::new();

        let report = gc.run_gc(&reachable, false).await.unwrap();
        assert_eq!(report.deleted_blobs, 1);
        assert!(!sha_dir.join(digest).exists());
    }

    #[tokio::test]
    async fn test_gc_keeps_reachable() {
        let dir = tempfile::tempdir().unwrap();
        let blob_dir = dir.path().join("blobs");
        let sha_dir = blob_dir.join("sha256");
        tokio::fs::create_dir_all(&sha_dir).await.unwrap();

        let digest = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        tokio::fs::write(sha_dir.join(digest), b"data").await.unwrap();

        let gc = LocalGarbageCollector::new(&blob_dir);
        let mut reachable = HashSet::new();
        reachable.insert(ContentHash::new_sha256(digest).unwrap());

        let report = gc.run_gc(&reachable, false).await.unwrap();
        assert_eq!(report.deleted_blobs, 0);
        assert!(sha_dir.join(digest).exists());
    }
}

/// Integration test: full image lifecycle from import through COW through GC.
#[cfg(test)]
mod integration_tests {
    use plaza_image::store::blob::{BlobStore, LocalBlobStore};
    use plaza_image::store::manifest::{ManifestStore, LocalManifestStore};
    use plaza_image::block::{FileBackedImmutableLayer, CowWritableLayer, VirtualBlockDevice, ImmutableLayer};
    use plaza_image::composer::LayeredBlockDevice;
    use plaza_image::gc::{GarbageCollector, LocalGarbageCollector};
    use plaza_image::model::*;
    use plaza_foundation::core::types::Timestamp;
    use std::collections::{HashMap, HashSet};
    use std::io::Cursor;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_full_storage_lifecycle() {
        let dir = tempfile::tempdir().unwrap();
        let blob_dir = dir.path().join("blobs");
        let manifest_dir = dir.path().join("manifests");

        let blob_store = LocalBlobStore::new(&blob_dir).await.unwrap();
        let manifest_store = LocalManifestStore::new(&manifest_dir).await.unwrap();

        // 1. Create base blob (8 KiB of pattern data)
        let mut base_data = vec![0u8; 8192];
        for (i, b) in base_data.iter_mut().enumerate() {
            *b = (i % 256) as u8;
        }
        let mut reader = Cursor::new(base_data.as_slice());
        let base_hash = blob_store.put_stream(&mut reader).await.unwrap();
        assert!(blob_store.exists(&base_hash).await.unwrap());

        // 2. Create second layer blob (4 KiB of 0xFF)
        let layer2_data = vec![0xFF; 4096];
        let mut reader2 = Cursor::new(layer2_data.as_slice());
        let layer2_hash = blob_store.put_stream(&mut reader2).await.unwrap();

        // 3. Create manifest
        let manifest = ImageManifest {
            version: 1,
            name: "test-lifecycle".to_string(),
            image_version: "1.0".to_string(),
            architecture: "x86_64".to_string(),
            layers: vec![
                ImageLayer {
                    digest: base_hash.clone(),
                    size: 8192,
                    media_type: LayerMediaType::RawBlock,
                },
                ImageLayer {
                    digest: layer2_hash.clone(),
                    size: 4096,
                    media_type: LayerMediaType::RawBlock,
                },
            ],
            kernel: None,
            initrd: None,
            boot: BootMetadata::default(),
            metadata: ImageMetadata {
                created_at: Timestamp::now(),
                author: None,
                labels: HashMap::new(),
            },
        };

        // 4. Register manifest
        manifest_store.put_manifest(&manifest).await.unwrap();

        // 5. Resolve manifest
        let resolved = manifest_store
            .get_manifest("test-lifecycle", "1.0")
            .await
            .unwrap()
            .expect("Manifest should exist");
        assert_eq!(resolved.layers.len(), 2);

        // 6. Build layered block device
        let base_path = blob_store.get_path(&base_hash).unwrap();
        let layer2_path = blob_store.get_path(&layer2_hash).unwrap();

        let base_layer = Arc::new(FileBackedImmutableLayer::open(base_path).await.unwrap());
        let over_layer = Arc::new(FileBackedImmutableLayer::open(layer2_path).await.unwrap());

        let cow_path = dir.path().join("workspace_cow.raw");
        let cow = CowWritableLayer::create(cow_path.clone(), 8192).await.unwrap();

        let mut device = LayeredBlockDevice::new(vec![base_layer, over_layer], cow);
        assert_eq!(device.size(), 8192);

        // 7. Read existing block from override layer (block 0 should be 0xFF from layer2)
        let mut buf = vec![0u8; 1];
        device.read_at(0, &mut buf).await.unwrap();
        assert_eq!(buf[0], 0xFF, "Override layer should win for first 4096 bytes");

        // 8. Read block 1 (offset 4096) — should be from base layer (0 % 256 = 0)
        let mut buf2 = vec![0u8; 1];
        device.read_at(4096, &mut buf2).await.unwrap();
        assert_eq!(buf2[0], 0, "Block 1 should come from base layer");

        // 9. Write to block 0
        device.write_at(0, b"PLAZAVM").await.unwrap();

        // 10. Read back modified block
        let mut buf3 = vec![0u8; 7];
        device.read_at(0, &mut buf3).await.unwrap();
        assert_eq!(&buf3, b"PLAZAVM");

        // 11. Verify immutable base layer hasn't changed
        let base_raw = tokio::fs::read(blob_store.get_path(&base_hash).unwrap()).await.unwrap();
        assert_eq!(base_raw[0], 0, "Base blob must remain immutable");

        // 12. Flush
        device.flush().await.unwrap();

        // 13. GC dry-run — all blobs should be "reachable" if we provide the hashes
        let gc = LocalGarbageCollector::new(&blob_dir);
        let mut reachable = HashSet::new();
        reachable.insert(base_hash.clone());
        reachable.insert(layer2_hash.clone());

        let report = gc.run_gc(&reachable, true).await.unwrap();
        assert_eq!(report.deleted_blobs, 0, "All blobs are reachable — nothing to delete");
    }
}
