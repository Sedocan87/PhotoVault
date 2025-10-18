import React, { useState } from "react";
import Library from "./pages/Library";
import SettingsPage from "./pages/Settings";
import { AlbumManager } from "./components/AlbumManager";
import { AlbumView } from "./pages/AlbumView";
import StatusBar from "./components/StatusBar";
import { Button } from "./components/ui/button";
import { Toaster } from "sonner";
import "./index.css";
import { DuplicateInspector } from "./components/DuplicateInspector";

type View = "library" | "settings" | "albums" | "duplicates";

function App() {
  const [view, setView] = useState<View>("library");
  const [selectedAlbumId, setSelectedAlbumId] = useState<number | null>(null);

  const handleAlbumSelect = (albumId: number) => {
    setSelectedAlbumId(albumId);
    setView("library"); // Or a new 'album-details' view
  };

  const renderView = () => {
    if (selectedAlbumId) {
      return <AlbumView albumId={selectedAlbumId} />;
    }
    switch (view) {
      case "settings":
        return <SettingsPage />;
      case "albums":
        return <AlbumManager onAlbumSelect={handleAlbumSelect} />;
      case "duplicates":
        return <DuplicateInspector />;
      case "library":
      default:
        return <Library />;
    }
  };

  return (
    <div className="flex flex-col h-screen bg-background text-foreground">
      <Toaster />
      <header className="p-2 border-b">
        <nav className="flex gap-2">
          <Button
            variant={view === "library" ? "secondary" : "ghost"}
            onClick={() => {
              setView("library");
              setSelectedAlbumId(null);
            }}
          >
            Library
          </Button>
          <Button
            variant={view === "albums" ? "secondary" : "ghost"}
            onClick={() => {
              setView("albums");
              setSelectedAlbumId(null);
            }}
          >
            Albums
          </Button>
          <Button
            variant={view === "duplicates" ? "secondary" : "ghost"}
            onClick={() => {
              setView("duplicates");
              setSelectedAlbumId(null);
            }}
          >
            Duplicates
          </Button>
          <Button
            variant={view === "settings" ? "secondary" : "ghost"}
            onClick={() => {
              setView("settings");
              setSelectedAlbumId(null);
            }}
          >
            Settings
          </Button>
        </nav>
      </header>
      <main className="flex-grow overflow-auto">{renderView()}</main>
      <StatusBar />
    </div>
  );
}

export default App;
