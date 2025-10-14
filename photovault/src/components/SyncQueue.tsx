import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface QueueStatus {
  pending_operations: number;
  is_sync_in_progress: boolean;
}

const SyncQueue: React.FC = () => {
  const [status, setStatus] = useState<QueueStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  const fetchStatus = async () => {
    try {
      const queueStatus: QueueStatus = await invoke('get_sync_queue_status');
      setStatus(queueStatus);
      setError(null);
    } catch (err) {
      setError(err as string);
    }
  };

  useEffect(() => {
    const interval = setInterval(fetchStatus, 5000); // Poll every 5 seconds
    fetchStatus(); // Initial fetch
    return () => clearInterval(interval);
  }, []);

  const handleRetry = async () => {
    try {
      await invoke('flush_sync_queue');
      fetchStatus();
    } catch (err) {
      setError(err as string);
    }
  };

  return (
    <div className="fixed bottom-4 right-4 bg-gray-800 text-white p-4 rounded-lg shadow-lg">
      <h3 className="text-lg font-bold mb-2">Sync Status</h3>
      {error && <p className="text-red-500">Error: {error}</p>}
      {status && (
        <div>
          <p>Pending Operations: {status.pending_operations}</p>
          <p>Sync in Progress: {status.is_sync_in_progress ? 'Yes' : 'No'}</p>
          {status.pending_operations > 0 && (
            <button
              onClick={handleRetry}
              className="mt-2 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            >
              Retry Failed
            </button>
          )}
        </div>
      )}
    </div>
  );
};

export default SyncQueue;