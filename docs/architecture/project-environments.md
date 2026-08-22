# Project Environments

PlazaVM strictly isolates the Host OS from the Project Environment.

```
HOST OS (Windows, macOS, Linux)
       ↓
PlazaVM CLI / Desktop
       ↓
plaza.yaml (Constraints)
       ↓
Workspace Image (Isolated)
       ↓
Project Environment
```

The Project Environment is constructed exclusively from the contents of the `Workspace Image`. 

## Immutability vs Mutability
The `Workspace Image` provides the base immutable layer for the project environment (compilers, interpreters, base dependencies). 

When a project environment is instantiated, PlazaVM provisions a `CowWritableLayer` (Copy-on-Write) on top of the image. All changes made during the session (temporary files, compiled objects, cached downloads) occur in the mutable layer, preserving the pristine state of the Workspace Image.
