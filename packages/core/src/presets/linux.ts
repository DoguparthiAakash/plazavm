import { Preset } from './windows';

const linuxDefaults = {
  guestOS: 'linux' as const,
  arch: 'x86_64' as const,
  cpu: { cores: 2, model: 'host' },
  memory: { mb: 2048 },
  disks: [{ path: 'disk0.qcow2', sizeGb: 32, bus: 'virtio' as const }],
  network: { mode: 'nat' as const },
  display: 'spice' as const,
  state: 'stopped' as const
};

export const LinuxPresets: Preset[] = [
  {
    id: 'ubuntu-2404',
    label: 'Ubuntu 24.04 LTS',
    icon: 'ubuntu',
    defaults: { ...linuxDefaults, id: '', name: 'Ubuntu' }
  },
  {
    id: 'fedora-40',
    label: 'Fedora 40',
    icon: 'fedora',
    defaults: { ...linuxDefaults, id: '', name: 'Fedora' }
  },
  {
    id: 'debian-12',
    label: 'Debian 12',
    icon: 'debian',
    defaults: { ...linuxDefaults, id: '', name: 'Debian' }
  },
  {
    id: 'arch',
    label: 'Arch Linux',
    icon: 'arch',
    defaults: { ...linuxDefaults, id: '', name: 'Arch Linux' }
  },
  {
    id: 'alpine',
    label: 'Alpine Linux',
    icon: 'alpine',
    defaults: { ...linuxDefaults, id: '', name: 'Alpine', memory: { mb: 512 }, disks: [{ path: 'disk0.qcow2', sizeGb: 8, bus: 'virtio' }] }
  }
];
