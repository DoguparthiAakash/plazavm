import { Preset } from './windows';

const bsdDefaults = {
  guestOS: 'bsd' as const,
  arch: 'x86_64' as const,
  cpu: { cores: 2, model: 'host' },
  memory: { mb: 2048 },
  disks: [{ path: 'disk0.qcow2', sizeGb: 20, bus: 'virtio' as const }],
  network: { mode: 'nat' as const },
  display: 'headless' as const,
  state: 'stopped' as const
};

export const BSDPresets: Preset[] = [
  {
    id: 'freebsd-14',
    label: 'FreeBSD 14',
    icon: 'freebsd',
    defaults: { ...bsdDefaults, id: '', name: 'FreeBSD' }
  },
  {
    id: 'openbsd-75',
    label: 'OpenBSD 7.5',
    icon: 'openbsd',
    defaults: { ...bsdDefaults, id: '', name: 'OpenBSD' }
  },
  {
    id: 'netbsd-10',
    label: 'NetBSD 10',
    icon: 'netbsd',
    defaults: { ...bsdDefaults, id: '', name: 'NetBSD' }
  }
];
