export interface ArchProfile {
  id: string; // e.g. 'x86_64', 'arm64', 'riscv64', 'mips64'
  label: string;
  qemuBinary: string; // e.g. 'qemu-system-x86_64'
  defaultMachine: string; // e.g. 'q35', 'virt', 'virt'
  defaultCPU: string; // e.g. 'host', 'cortex-a57', 'rv64'
  supportedGuests: Array<'windows' | 'linux' | 'bsd'>;
  notes?: string; // human-readable caveats
}

export const archRegistry: ArchProfile[] = [
  {
    id: 'x86_64',
    label: 'x86 (64-bit)',
    qemuBinary: 'qemu-system-x86_64',
    defaultMachine: 'q35',
    defaultCPU: 'host',
    supportedGuests: ['windows', 'linux', 'bsd'],
  },
  {
    id: 'arm64',
    label: 'ARM64',
    qemuBinary: 'qemu-system-aarch64',
    defaultMachine: 'virt',
    defaultCPU: 'cortex-a57',
    supportedGuests: ['linux'],
    notes: 'Windows on ARM not fully verified yet',
  },
  {
    id: 'riscv64',
    label: 'RISC-V (64-bit)',
    qemuBinary: 'qemu-system-riscv64',
    defaultMachine: 'virt',
    defaultCPU: 'rv64',
    supportedGuests: [],
    notes: 'Experimental CPU target',
  }
];
