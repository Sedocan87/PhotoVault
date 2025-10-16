import React from 'react';
import { Photo } from '../models/photo';

interface GalleryProps {
  photos: Photo[];
  selectedPhotoIds?: number[];
  setSelectedPhotoIds?: React.Dispatch<React.SetStateAction<number[]>>;
}

const Gallery: React.FC<GalleryProps> = ({ photos, selectedPhotoIds, setSelectedPhotoIds }) => {
  const handlePhotoSelection = (photoId: number) => {
    if (setSelectedPhotoIds) {
      setSelectedPhotoIds((prev) =>
        prev.includes(photoId) ? prev.filter((id) => id !== photoId) : [...prev, photoId]
      );
    }
  };

  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4">
      {photos.map((photo) => (
        <div key={photo.id} className="relative group">
          <div
            className={`bg-gray-200 dark:bg-gray-700 aspect-square rounded-md group-hover:opacity-80 ${
              selectedPhotoIds?.includes(photo.id) ? 'ring-2 ring-blue-500' : ''
            }`}
            onClick={() => handlePhotoSelection(photo.id)}
          >
            <p className="text-white">{photo.filename}</p>
          </div>
          {selectedPhotoIds && (
            <input
              type="checkbox"
              checked={selectedPhotoIds.includes(photo.id)}
              onChange={() => handlePhotoSelection(photo.id)}
              className="absolute top-2 left-2"
            />
          )}
        </div>
      ))}
    </div>
  );
};

export default Gallery;