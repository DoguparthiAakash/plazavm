import React from 'react';
import { twMerge } from 'tailwind-merge';

interface StatusBadgeProps {
  status: 'stopped' | 'running' | 'paused' | 'error';
}

export const StatusBadge: React.FC<StatusBadgeProps> = ({ status }) => {
  const colors = {
    running: 'bg-green-100 text-green-800 border-green-200',
    stopped: 'bg-gray-100 text-gray-800 border-gray-200',
    paused: 'bg-yellow-100 text-yellow-800 border-yellow-200',
    error: 'bg-red-100 text-red-800 border-red-200',
  };

  return (
    <span className={twMerge('px-2 py-1 text-xs font-medium rounded border', colors[status])}>
      {status.charAt(0).toUpperCase() + status.slice(1)}
    </span>
  );
};
