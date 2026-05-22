import React, { useState } from 'react';

export const CreateWizard: React.FC = () => {
  const [step, setStep] = useState(1);
  const [os, setOs] = useState<string | null>(null);

  const handleNext = () => setStep(s => s + 1);
  const handleBack = () => setStep(s => s - 1);

  return (
    <div className="fixed right-0 top-0 h-full w-96 bg-white dark:bg-gray-900 shadow-xl border-l p-6 transition-transform">
      <h2 className="text-2xl font-bold mb-6">Create New VM</h2>
      
      {step === 1 && (
        <div>
          <h3 className="mb-4">Step 1: Choose Guest OS</h3>
          <div className="grid grid-cols-1 gap-2">
            <button onClick={() => setOs('windows')} className="p-4 border rounded">Windows</button>
            <button onClick={() => setOs('linux')} className="p-4 border rounded">Linux</button>
            <button onClick={() => setOs('bsd')} className="p-4 border rounded">BSD</button>
          </div>
          {os && <button onClick={handleNext} className="mt-4 px-4 py-2 bg-blue-600 text-white rounded w-full">Next</button>}
        </div>
      )}

      {step === 2 && (
        <div>
          <h3 className="mb-4">Step 2: Configure</h3>
          <p>Preset options for {os} go here.</p>
          <div className="flex gap-2 mt-4">
            <button onClick={handleBack} className="px-4 py-2 border rounded w-full">Back</button>
            <button onClick={handleNext} className="px-4 py-2 bg-blue-600 text-white rounded w-full">Next</button>
          </div>
        </div>
      )}

      {step === 3 && (
        <div>
          <h3 className="mb-4">Step 3: Creating...</h3>
          <div className="w-full bg-gray-200 rounded-full h-2.5 mb-4 dark:bg-gray-700">
            <div className="bg-blue-600 h-2.5 rounded-full w-1/3"></div>
          </div>
          <p>Provisioning disk image...</p>
        </div>
      )}
    </div>
  );
};
