import React from 'react';
import Sidebar from '../components/Sidebar';
import Gallery from '../components/Gallery';

const Library: React.FC = () => {
  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 p-4 overflow-y-auto">
        <Gallery />
      </main>
    </div>
  );
};

export default Library;