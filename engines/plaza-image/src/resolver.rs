use crate::model::{ContentHash, ImageRef};
use plaza_foundation::core::{PlazaError, PlazaResult};

/// Parses an image reference string into an `ImageRef` struct.
/// Supported formats:
/// - `name` -> ImageRef { name: "name", tag: Some("latest"), digest: None }
/// - `name:tag` -> ImageRef { name: "name", tag: Some("tag"), digest: None }
/// - `name@sha256:digest` -> ImageRef { name: "name", tag: None, digest: Some(...) }
/// - `name:tag@sha256:digest` -> ImageRef { name: "name", tag: Some("tag"), digest: Some(...) }
pub fn parse_image_ref(reference: &str) -> PlazaResult<ImageRef> {
    if reference.is_empty() {
        return Err(PlazaError::InvalidImageReference("Reference is empty".into()));
    }

    let mut name = reference;
    let mut tag = None;
    let mut digest = None;

    if let Some((rest, digest_str)) = reference.split_once('@') {
        let h = digest_str.parse::<ContentHash>().map_err(|_| {
            PlazaError::InvalidImageReference(format!("Invalid digest: {}", digest_str))
        })?;
        digest = Some(h);
        name = rest;
    }

    if let Some((n, t)) = name.split_once(':') {
        name = n;
        tag = Some(t.to_string());
    } else if digest.is_none() {
        // Default to latest if no digest and no tag provided
        tag = Some("latest".to_string());
    }

    if name.is_empty() {
        return Err(PlazaError::InvalidImageReference("Missing image name".into()));
    }

    Ok(ImageRef {
        name: name.to_string(),
        tag,
        digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name() {
        let r = parse_image_ref("alpine").unwrap();
        assert_eq!(r.name, "alpine");
        assert_eq!(r.tag.unwrap(), "latest");
        assert!(r.digest.is_none());
    }

    #[test]
    fn test_parse_name_tag() {
        let r = parse_image_ref("alpine:3.18").unwrap();
        assert_eq!(r.name, "alpine");
        assert_eq!(r.tag.unwrap(), "3.18");
        assert!(r.digest.is_none());
    }

    #[test]
    fn test_parse_name_digest() {
        let hash = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let r = parse_image_ref(&format!("alpine@{}", hash)).unwrap();
        assert_eq!(r.name, "alpine");
        assert!(r.tag.is_none());
        assert_eq!(r.digest.unwrap().to_string(), hash);
    }
}
