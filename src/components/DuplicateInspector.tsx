import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { Button } from "./ui/button";
import { Checkbox } from "@/components/ui/checkbox";

interface Photo {
  id: number;
  path: string;
  filename: string;
  file_size: number;
}

interface DuplicateGroup {
  hash: string;
  photos: Photo[];
  size: number;
}

export function DuplicateInspector() {
  const [duplicateGroups, setDuplicateGroups] = useState<DuplicateGroup[]>([]);
  const [selectedPhotos, setSelectedPhotos] = useState<number[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<DuplicateGroup[]>("find_duplicates")
      .then(setDuplicateGroups)
      .catch((err) => setError(err.toString()));
  }, []);

  const handleSelectPhoto = (photoId: number) => {
    setSelectedPhotos((prev) =>
      prev.includes(photoId)
        ? prev.filter((id) => id !== photoId)
        : [...prev, photoId],
    );
  };

  const handleDeleteSelected = async () => {
    try {
      const spaceFreed = await invoke<number>("delete_duplicates", {
        photoIds: selectedPhotos,
      });
      alert(
        `Successfully deleted photos and freed ${(spaceFreed / 1024 / 1024).toFixed(2)} MB`,
      );
      // Refresh the list of duplicates
      invoke<DuplicateGroup[]>("find_duplicates").then(setDuplicateGroups);
      setSelectedPhotos([]);
    } catch (err) {
      setError((err as Error).toString());
    }
  };

  const totalSelectedSize = duplicateGroups
    .flatMap((group) => group.photos)
    .filter((photo) => selectedPhotos.includes(photo.id))
    .reduce((acc, photo) => acc + photo.file_size, 0);

  if (error) {
    return <div className="text-red-500">Error: {error}</div>;
  }

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <h1 className="text-2xl font-bold">Duplicate Photos</h1>
        <div className="flex items-center gap-4">
          <span>
            {`Selected: ${(totalSelectedSize / 1024 / 1024).toFixed(2)} MB`}
          </span>
          <Button
            onClick={handleDeleteSelected}
            disabled={selectedPhotos.length === 0}
          >
            Delete Selected
          </Button>
        </div>
      </div>
      {duplicateGroups.length === 0 ? (
        <p>No duplicates found.</p>
      ) : (
        <div>
          {duplicateGroups.map((group) => (
            <div key={group.hash} className="mb-8 p-4 border rounded-lg">
              <h2 className="text-xl font-semibold mb-2">
                Duplicate Set - Save {(group.size / 1024 / 1024).toFixed(2)} MB
              </h2>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {group.photos.map((photo) => (
                  <div key={photo.id} className="relative">
                    <img
                      src={`https://asset.localhost/${photo.path}`}
                      alt={photo.filename}
                      className="w-full h-auto rounded-md"
                    />
                    <div className="absolute top-2 left-2">
                      <Checkbox
                        checked={selectedPhotos.includes(photo.id)}
                        onCheckedChange={() => handleSelectPhoto(photo.id)}
                      />
                    </div>
                    <p className="mt-2 text-sm text-center">{photo.filename}</p>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
