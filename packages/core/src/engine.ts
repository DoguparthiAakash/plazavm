import { EventEmitter } from 'events';
import { VMConfig } from './config';

export interface VMEngineEvents {
  'vm:started': (id: string) => void;
  'vm:stopped': (id: string) => void;
  'vm:error': (id: string, error: Error) => void;
  'vm:progress': (id: string, progress: number) => void;
}

export declare interface VMEngine {
  on<U extends keyof VMEngineEvents>(event: U, listener: VMEngineEvents[U]): this;
  emit<U extends keyof VMEngineEvents>(event: U, ...args: Parameters<VMEngineEvents[U]>): boolean;
}

export class VMEngine extends EventEmitter {
  constructor() {
    super();
  }

  async create(config: VMConfig): Promise<void> {
    // TODO: Write config to ~/.plazavm/vms/{id}.json
    // TODO: Provision disks
  }

  async start(id: string): Promise<void> {
    // TODO: Build QEMU command and spawn child process
    this.emit('vm:started', id);
  }

  async stop(id: string): Promise<void> {
    // TODO: Send graceful shutdown signal to QEMU monitor
    this.emit('vm:stopped', id);
  }

  async pause(id: string): Promise<void> {
    // TODO: Pause VM execution via QEMU monitor
  }

  async resume(id: string): Promise<void> {
    // TODO: Resume VM execution
  }

  async delete(id: string): Promise<void> {
    // TODO: Delete config and virtual disk files
  }

  async snapshot(id: string, name: string): Promise<void> {
    // TODO: Create QCOW2 snapshot
  }

  async listVMs(): Promise<VMConfig[]> {
    // TODO: Read configs from ~/.plazavm/vms/
    return [];
  }
}
