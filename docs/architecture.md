# PlazaVM Architecture

PlazaVM uses an Electron + React frontend, communicating with a Node.js main process. The main process imports the `@plazavm/core` TS library, which manages QEMU subprocesses and disk images.

## Packages
- `@plazavm/core`: QEMU bindings, state management, configuration parsing.
- `@plazavm/ui-kit`: Reusable React components.

## Apps
- `@plazavm/desktop`: Electron app providing the GUI.
- `@plazavm/cli`: Headless CLI consuming `@plazavm/core`.
