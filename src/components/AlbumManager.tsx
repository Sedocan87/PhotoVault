import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

interface Album {
  id: number;
  name: string;
}

interface AlbumManagerProps {
  onAlbumSelect: (albumId: number) => void;
}

export function AlbumManager({ onAlbumSelect }: AlbumManagerProps) {
  const queryClient = useQueryClient();
  const [albumName, setAlbumName] = useState("");

  const { data: albums, isLoading } = useQuery<Album[]>({
    queryKey: ["albums"],
    queryFn: () => invoke("get_albums"),
  });

  const createAlbumMutation = useMutation({
    mutationFn: (name: string) => invoke("create_album", { name }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["albums"] });
      setAlbumName("");
    },
  });

  const handleCreateAlbum = () => {
    if (albumName.trim()) {
      createAlbumMutation.mutate(albumName);
    }
  };

  const deleteAlbumMutation = useMutation({
    mutationFn: (albumId: number) => invoke("delete_album", { albumId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["albums"] });
    },
  });

  return (
    <div className="p-4">
      <h2 className="text-lg font-bold mb-4">Album Management</h2>
      <div className="flex gap-2 mb-4">
        <input
          type="text"
          value={albumName}
          onChange={(e) => setAlbumName(e.target.value)}
          placeholder="New album name"
          className="border p-2 rounded w-full"
        />
        <button
          onClick={handleCreateAlbum}
          disabled={createAlbumMutation.isPending}
          className="bg-blue-500 text-white p-2 rounded"
        >
          {createAlbumMutation.isPending ? "Creating..." : "Create Album"}
        </button>
      </div>
      {isLoading ? (
        <p>Loading albums...</p>
      ) : (
        <ul>
          {albums?.map((album) => (
            <li
              key={album.id}
              className="p-2 border-b flex justify-between items-center"
            >
              <span
                onClick={() => onAlbumSelect(album.id)}
                className="cursor-pointer hover:underline"
              >
                {album.name}
              </span>
              <button
                onClick={() => deleteAlbumMutation.mutate(album.id)}
                disabled={deleteAlbumMutation.isPending}
                className="bg-red-500 text-white p-1 rounded"
              >
                Delete
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
