import { z } from 'zod';

export const VMConfigSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  guestOS: z.enum(['windows', 'linux', 'bsd']),
  arch: z.enum(['x86_64', 'arm64', 'riscv64']),
  cpu: z.object({
    cores: z.number().int().positive(),
    model: z.string(),
  }),
  memory: z.object({
    mb: z.number().int().positive(),
  }),
  disks: z.array(z.object({
    path: z.string(),
    sizeGb: z.number().positive(),
    bus: z.enum(['virtio', 'ide', 'nvme']),
  })),
  network: z.object({
    mode: z.enum(['nat', 'bridged', 'hostonly']),
    interface: z.string().optional(),
  }),
  display: z.enum(['sdl', 'vnc', 'spice', 'headless']),
  state: z.enum(['stopped', 'running', 'paused', 'error']).default('stopped'),
});

export type VMConfig = z.infer<typeof VMConfigSchema>;
