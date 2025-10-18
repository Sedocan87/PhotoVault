import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog';

interface DriveSetupModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfigSaved: () => void;
}

const DriveSetupModal: React.FC<DriveSetupModalProps> = ({ isOpen, onClose, onConfigSaved }) => {
  const [primaryDrive, setPrimaryDrive] = useState<string | null>(null);
  const [backupDrive, setBackupDrive] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const selectDirectory = async (setter: React.Dispatch<React.SetStateAction<string | null>>) => {
    const result = await open({
      directory: true,
      multiple: false,
      title: 'Select a directory',
    });
    if (typeof result === 'string') {
      setter(result);
    }
  };

  const handleSave = async () => {
    if (!primaryDrive || !backupDrive) {
      setError('Both primary and backup drives must be selected.');
      return;
    }

    setIsSaving(true);
    setError(null);

    try {
      await invoke('set_drive_paths', { primary: primaryDrive, backup: backupDrive });
      onConfigSaved();
      onClose();
    } catch (err) {
      setError(err as string);
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Configure Drives</DialogTitle>
          <DialogDescription>
            Select a primary drive for your photo library and a backup drive for synchronization.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <div className="flex items-center gap-4">
            <Button variant="outline" onClick={() => selectDirectory(setPrimaryDrive)} className="flex-grow">
              Select Primary Drive
            </Button>
            <p className="text-sm text-muted-foreground truncate">{primaryDrive || 'No directory selected'}</p>
          </div>
          <div className="flex items-center gap-4">
            <Button variant="outline" onClick={() => selectDirectory(setBackupDrive)} className="flex-grow">
              Select Backup Drive
            </Button>
            <p className="text-sm text-muted-foreground truncate">{backupDrive || 'No directory selected'}</p>
          </div>
        </div>

        {error && <p className="text-sm text-red-500">{error}</p>}

        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>Cancel</Button>
          <Button onClick={handleSave} disabled={isSaving || !primaryDrive || !backupDrive}>
            {isSaving ? 'Saving...' : 'Save Configuration'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default DriveSetupModal;