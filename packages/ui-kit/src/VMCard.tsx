import React from 'react';
import { StatusBadge } from './StatusBadge';

export interface VMCardProps {
  id: string;
  name: string;
  osIcon: React.ReactNode;
  ramMb: number;
  status: 'stopped' | 'running' | 'paused' | 'error';
  onStart: () => void;
  onStop: () => void;
  onSettings: () => void;
}

export const VMCard: React.FC<VMCardProps> = ({ name, osIcon, ramMb, status, onStart, onStop, onSettings }) => {
  return (
    <div className="flex flex-col p-4 border rounded-lg shadow-sm bg-white dark:bg-gray-800">
      <div className="flex justify-between items-center mb-4">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8">{osIcon}</div>
          <h3 className="text-lg font-semibold">{name}</h3>
        </div>
        <StatusBadge status={status} />
      </div>
      <div className="text-sm text-gray-500 mb-4">
        RAM: {ramMb} MB
      </div>
      <div className="flex gap-2 mt-auto">
        {status === 'stopped' ? (
          <button onClick={onStart} className="px-3 py-1 bg-blue-600 text-white rounded">Start</button>
        ) : (
          <button onClick={onStop} className="px-3 py-1 bg-red-600 text-white rounded">Stop</button>
        )}
        <button onClick={onSettings} className="px-3 py-1 bg-gray-200 text-gray-800 rounded">Settings</button>
      </div>
    </div>
  );
};
