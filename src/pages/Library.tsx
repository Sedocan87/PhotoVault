import React, { useState } from "react";
import Sidebar from "../components/Sidebar";
import Gallery from "../components/Gallery";
import { BulkActions } from "../components/BulkActions";
import { useInfiniteQuery, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";
import { Album } from "../models/album";
import { Photo } from "../models/photo";

const fetchPhotos = async ({ pageParam = 0 }) => {
  const photos: Photo[] = await invoke("get_photos", {
    limit: 20,
    offset: pageParam * 20,
  });
  return { photos, nextPage: pageParam + 1 };
};

const Library: React.FC = () => {
  const [selectedPhotoIds, setSelectedPhotoIds] = useState<number[]>([]);

  const { data: albums } = useQuery<Album[]>({
    queryKey: ["albums"],
    queryFn: () => invoke("get_albums"),
  });

  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
    isLoading,
    isError,
  } = useInfiniteQuery({
    queryKey: ["photos"],
    queryFn: fetchPhotos,
    getNextPageParam: (lastPage) =>
      lastPage.photos.length > 0 ? lastPage.nextPage : undefined,
    initialPageParam: 0,
  });

  const allPhotos = data?.pages.flatMap((page) => page.photos) || [];

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-grow overflow-hidden">
        <Sidebar />
        <main className="flex-1 p-4 overflow-y-auto">
          {isLoading && <p>Loading photos...</p>}
          {isError && <p>Error loading photos.</p>}
          <Gallery
            photos={allPhotos}
            selectedPhotoIds={selectedPhotoIds}
            setSelectedPhotoIds={setSelectedPhotoIds}
          />
          {hasNextPage && (
            <button
              onClick={() => fetchNextPage()}
              disabled={isFetchingNextPage}
              className="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            >
              {isFetchingNextPage ? "Loading more..." : "Load More"}
            </button>
          )}
        </main>
      </div>
      <BulkActions selectedPhotoIds={selectedPhotoIds} albums={albums || []} />
    </div>
  );
};

export default Library;
