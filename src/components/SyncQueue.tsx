import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface QueueStatus {
  pending_operations: number;
}

export function SyncQueue() {
  const [status, setStatus] = useState<QueueStatus | null>(null);

  useEffect(() => {
    const interval = setInterval(() => {
      invoke<QueueStatus>("get_sync_queue_status")
        .then(setStatus)
        .catch(console.error);
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  if (!status) {
    return null;
  }

  return (
    <div>
      <p>Pending operations: {status.pending_operations}</p>
    </div>
  );
}
