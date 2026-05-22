import { VMConfig } from '../config';

export interface Preset {
  id: string;
  label: string;
  icon: string;
  defaults: VMConfig;
  recommendedISOUrl?: string;
}

export const WindowsPresets: Preset[] = [
  {
    id: 'win11',
    label: 'Windows 11',
    icon: 'windows-11',
    defaults: {
      id: '', // Generated at creation
      name: 'Windows 11',
      guestOS: 'windows',
      arch: 'x86_64',
      cpu: { cores: 4, model: 'host' },
      memory: { mb: 4096 },
      disks: [
        { path: 'disk0.qcow2', sizeGb: 64, bus: 'nvme' }
      ],
      network: { mode: 'nat' },
      display: 'spice',
      state: 'stopped'
    },
    // Note: requires TPM 2.0 and virtio-win auto-attach
  },
  {
    id: 'win10',
    label: 'Windows 10',
    icon: 'windows-10',
    defaults: {
      id: '',
      name: 'Windows 10',
      guestOS: 'windows',
      arch: 'x86_64',
      cpu: { cores: 4, model: 'host' },
      memory: { mb: 4096 },
      disks: [
        { path: 'disk0.qcow2', sizeGb: 64, bus: 'nvme' }
      ],
      network: { mode: 'nat' },
      display: 'spice',
      state: 'stopped'
    }
  },
  {
    id: 'win2022',
    label: 'Windows Server 2022',
    icon: 'windows-server',
    defaults: {
      id: '',
      name: 'Windows Server 2022',
      guestOS: 'windows',
      arch: 'x86_64',
      cpu: { cores: 4, model: 'host' },
      memory: { mb: 4096 },
      disks: [
        { path: 'disk0.qcow2', sizeGb: 64, bus: 'nvme' }
      ],
      network: { mode: 'nat' },
      display: 'spice',
      state: 'stopped'
    }
  }
];
