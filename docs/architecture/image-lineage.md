# Image Lineage

Workspace Images trace their lineage to their cryptographic source.

Lineage tracks the deterministic identity computation:

1. `plaza.yaml` is parsed into a configuration tree.
2. The relevant sections (Engine, Project, Image, Environment) are extracted.
3. This creates a `WorkspaceImageSpec`.
4. The fields in `WorkspaceImageSpec` are normalized (sorted alphabetically to guarantee stable order).
5. The normalized struct is converted to canonical JSON.
6. A SHA-256 hash digest is generated from the JSON byte array.

Because the lineage is cryptographically bound to the configuration parameters, identical environments will ALWAYS yield the same digest and reuse the same base artifacts, preventing duplicate caching.
