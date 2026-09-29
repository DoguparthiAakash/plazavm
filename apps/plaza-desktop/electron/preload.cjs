const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
  invoke: (channel, ...args) => ipcRenderer.invoke(channel, ...args),

  // Live event streaming
  onEvent: (callback) => {
    ipcRenderer.on('plaza-event', (_, event) => callback(event));
  },
  removeEventListeners: () => {
    ipcRenderer.removeAllListeners('plaza-event');
  },
});
