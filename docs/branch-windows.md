# Branch: feature/windows

## Scope
- Support Windows guest VMs (Windows 10, 11, Server 2022).
- Automatically mount `virtio-win` ISO to provide virtio disk and network drivers.
- Provide TPM 2.0 emulation (`tpm-tis-device`) for Windows 11.

## Merge Criteria
- Windows 11 installs successfully with network access.
- SPICE display provides smooth UI.
