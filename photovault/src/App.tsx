import React, { useState } from 'react';
import SyncQueue from './components/SyncQueue';
import Toast from './components/Toast';

interface ToastState {
  message: string;
  type: 'success' | 'error';
}

function App() {
  const [toast, setToast] = useState<ToastState | null>(null);

  // Example function to show a toast
  const showToast = (message: string, type: 'success' | 'error') => {
    setToast({ message, type });
    setTimeout(() => setToast(null), 5000); // Hide after 5 seconds
  };

  return (
    <div className="container">
      <h1>Photovault</h1>
      <p>Your local photo management solution.</p>

      {/* Example buttons to trigger toasts */}
      <div className="my-4">
        <button onClick={() => showToast('Operation successful!', 'success')} className="bg-green-500 text-white p-2 rounded mr-2">
          Show Success Toast
        </button>
        <button onClick={() => showToast('Operation failed!', 'error')} className="bg-red-500 text-white p-2 rounded">
          Show Error Toast
        </button>
      </div>

      <SyncQueue />
      {toast && <Toast message={toast.message} type={toast.type} onClose={() => setToast(null)} />}
    </div>
  );
}

export default App;