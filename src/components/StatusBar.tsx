import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SyncQueue } from "./SyncQueue";

interface SyncStatus {
  primary_connected: boolean;
  backup_connected: boolean;
  is_in_sync: boolean;
  pending_operations: number;
}

const StatusBar: React.FC = () => {
  const [status, setStatus] = useState<SyncStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  const fetchStatus = async () => {
    try {
      const result = await invoke<SyncStatus>("verify_sync_status");
      setStatus(result);
      setError(null);
    } catch (err) {
      setError(err as string);
    }
  };

  useEffect(() => {
    // Fetch status immediately on mount
    fetchStatus();
    // And then fetch every 30 seconds
    const interval = setInterval(fetchStatus, 30000);
    return () => clearInterval(interval);
  }, []);

  const getStatusIndicator = () => {
    if (!status || !status.primary_connected) {
      return <span className="text-yellow-500">● Primary Disconnected</span>;
    }
    if (!status.backup_connected) {
      return <span className="text-red-500">● Backup Offline</span>;
    }
    if (status.is_in_sync) {
      return <span className="text-green-500">● In Sync</span>;
    }
    return <span className="text-gray-500">● Unknown</span>;
  };

  return (
    <footer className="p-2 border-t text-xs text-muted-foreground">
      <div className="flex justify-between items-center">
        <div>
          <SyncQueue />
        </div>
        <div className="flex items-center gap-2">
          {error && <span className="text-red-500">Error checking status</span>}
          {status ? getStatusIndicator() : <span>Loading status...</span>}
        </div>
      </div>
    </footer>
  );
};

export default StatusBar;
