import React from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { Button } from './ui/button';

interface BulkActionsProps {
  selectedPhotoIds: number[];
  albums: { id: number; name: string }[];
}

export function BulkActions({ selectedPhotoIds, albums }: BulkActionsProps) {
  const queryClient = useQueryClient();
  const [selectedAlbumId, setSelectedAlbumId] = React.useState<number | null>(null);

  const addToAlbumMutation = useMutation({
    mutationFn: ({ photoIds, albumId }: { photoIds: number[]; albumId: number }) =>
      invoke('add_photos_to_album', { photoIds, albumId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['photos'] });
      queryClient.invalidateQueries({ queryKey: ['albums'] });
    },
  });

  const handleAddToAlbum = () => {
    if (selectedAlbumId && selectedPhotoIds.length > 0) {
      addToAlbumMutation.mutate({ photoIds: selectedPhotoIds, albumId: selectedAlbumId });
    }
  };

  if (selectedPhotoIds.length === 0) {
    return null;
  }

  return (
    <div className="p-4 bg-gray-100 border-t">
      <p>{selectedPhotoIds.length} photos selected</p>
      <div className="flex gap-2 mt-2">
        <select
          onChange={(e) => setSelectedAlbumId(Number(e.target.value))}
          className="border p-2 rounded"
          defaultValue=""
        >
          <option value="" disabled>
            Select an album
          </option>
          {albums.map((album) => (
            <option key={album.id} value={album.id}>
              {album.name}
            </option>
          ))}
        </select>
        <Button onClick={handleAddToAlbum} disabled={!selectedAlbumId || addToAlbumMutation.isPending}>
          {addToAlbumMutation.isPending ? 'Adding...' : 'Add to Album'}
        </Button>
      </div>
    </div>
  );
}