import React from 'react';
import { useInfiniteQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';

interface Photo {
  id: number;
  path: string;
  filename: string;
}

const fetchPhotos = async ({ pageParam = 0 }) => {
  const photos: Photo[] = await invoke('get_photos', { limit: 20, offset: pageParam * 20 });
  return { photos, nextPage: pageParam + 1 };
};

const Gallery: React.FC = () => {
  const queryClient = useQueryClient();
  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
    isLoading,
    isError,
  } = useInfiniteQuery({
    queryKey: ['photos'],
    queryFn: fetchPhotos,
    getNextPageParam: (lastPage) => (lastPage.photos.length > 0 ? lastPage.nextPage : undefined),
    initialPageParam: 0,
  });

  const scanLibraryMutation = useMutation({
    mutationFn: async (path: string) => {
      await invoke('scan_library', { primaryPath: path });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['photos'] });
    },
  });

  const handleScan = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (typeof selected === 'string') {
      scanLibraryMutation.mutate(selected);
    }
  };

  return (
    <div>
      <div className="flex justify-between items-center mb-4">
        <h1 className="text-2xl font-bold">All Photos</h1>
        <button
          onClick={handleScan}
          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
          disabled={scanLibraryMutation.isPending}
        >
          {scanLibraryMutation.isPending ? 'Scanning...' : 'Scan Library'}
        </button>
      </div>

      {isLoading && <p>Loading photos...</p>}
      {isError && <p>Error loading photos.</p>}

      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4">
        {data?.pages.map((page, i) => (
          <React.Fragment key={i}>
            {page.photos.map((photo) => (
              <div key={photo.id} className="bg-gray-200 dark:bg-gray-700 aspect-square rounded-md">
                <p className="text-white">{photo.filename}</p>
              </div>
            ))}
          </React.Fragment>
        ))}
      </div>

      {hasNextPage && (
        <button
          onClick={() => fetchNextPage()}
          disabled={isFetchingNextPage}
          className="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        >
          {isFetchingNextPage ? 'Loading more...' : 'Load More'}
        </button>
      )}
    </div>
  );
};

export default Gallery;