import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import DriveSetupModal from "@/components/DriveSetupModal";

interface AppConfig {
  primary_drive: string | null;
  backup_drive: string | null;
}

const SettingsPage: React.FC = () => {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const fetchConfig = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const result = await invoke<AppConfig>("get_config");
      setConfig(result);
      // If this is the first run (no config), open the setup modal automatically
      if (!result.primary_drive || !result.backup_drive) {
        setIsModalOpen(true);
      }
    } catch (err) {
      setError(err as string);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchConfig();
  }, []);

  const handleConfigSaved = () => {
    fetchConfig(); // Re-fetch the config to display the new paths
  };

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-4">Settings</h1>

      <div className="space-y-4">
        <h2 className="text-xl font-semibold">Drive Configuration</h2>
        {isLoading && <p>Loading configuration...</p>}
        {error && <p className="text-red-500">Error: {error}</p>}

        {config && (
          <div className="p-4 border rounded-md bg-muted/50">
            <div className="space-y-2">
              <div>
                <p className="font-medium">Primary Drive:</p>
                <p className="text-sm text-muted-foreground">
                  {config.primary_drive || "Not set"}
                </p>
              </div>
              <div>
                <p className="font-medium">Backup Drive:</p>
                <p className="text-sm text-muted-foreground">
                  {config.backup_drive || "Not set"}
                </p>
              </div>
            </div>
          </div>
        )}

        <Button onClick={() => setIsModalOpen(true)}>Configure Drives</Button>
      </div>

      <DriveSetupModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        onConfigSaved={handleConfigSaved}
      />
    </div>
  );
};

export default SettingsPage;
