import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/tauri';
import { Photo } from '../models/photo';
import Gallery from '../components/Gallery';

interface AlbumViewProps {
  albumId: number;
}

export function AlbumView({ albumId }: AlbumViewProps) {
  const { data: photos, isLoading } = useQuery<Photo[]>({
    queryKey: ['album', albumId],
    queryFn: () => invoke('get_photos_by_album', { albumId }),
  });

  if (isLoading) {
    return <p>Loading photos...</p>;
  }

  return (
    <div className="p-4">
      <h2 className="text-lg font-bold mb-4">Album</h2>
      <Gallery photos={photos || []} />
    </div>
  );
}