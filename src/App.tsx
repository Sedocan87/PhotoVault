import React, { useState } from 'react';
import Library from './pages/Library';
import SettingsPage from './pages/Settings';
import StatusBar from './components/StatusBar';
import { Button } from './components/ui/button';
import { Toaster } from 'sonner';
import './index.css';

type View = 'library' | 'settings';

function App() {
  const [view, setView] = useState<View>('library');

  const renderView = () => {
    switch (view) {
      case 'settings':
        return <SettingsPage />;
      case 'library':
      default:
        return <Library />;
    }
  };

  return (
    <div className="flex flex-col h-screen bg-background text-foreground">
      <Toaster />
      <header className="p-2 border-b">
        <nav className="flex gap-2">
          <Button variant={view === 'library' ? 'secondary' : 'ghost'} onClick={() => setView('library')}>
            Library
          </Button>
          <Button variant={view === 'settings' ? 'secondary' : 'ghost'} onClick={() => setView('settings')}>
            Settings
          </Button>
        </nav>
      </header>
      <main className="flex-grow overflow-auto">
        {renderView()}
      </main>
      <StatusBar />
    </div>
  );
}

export default App;