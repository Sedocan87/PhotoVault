# PhotoVault - Architecture & Design

## Tech Stack

### Frontend
- **Framework**: React 18+ with TypeScript
- **Styling**: Tailwind CSS + shadcn/ui component library
- **State Management**: TanStack Query (React Query) for server state, Zustand for client state
- **Build Tool**: Vite (fast HMR, optimized builds)
- **IPC**: Tauri's command-based system for backend communication

### Backend
- **Runtime**: Tauri + Rust
- **Async**: Tokio for concurrent operations
- **File Operations**: Walkdir, std::fs, async file handling
- **Image Processing**: Image crate for metadata reading, dimensions, thumbnails
- **Database**: SQLite with sqlx for async queries, mirrored to both drives
- **Hashing**: Sha256 for duplicate detection across drives

### Infrastructure
- **Desktop Runtime**: Tauri 2.0+
- **OS Support**: Windows (MSVC), macOS (Universal), Linux (GTK)
- **Binary Size Target**: ~50MB uncompressed per platform

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                  React Frontend (UI)                │
│            (shadcn/ui + Tailwind CSS)               │
└──────────────────────┬──────────────────────────────┘
                       │ Tauri Commands (JSON-RPC)
                       ▼
┌─────────────────────────────────────────────────────┐
│              Tauri Runtime Bridge                   │
│         (IPC between Frontend & Backend)            │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│              Rust Backend Services                  │
│  ┌────────────┐  ┌────────────┐  ┌──────────────┐ │
│  │ File Ops   │  │ Sync Engine│  │ DB Manager   │ │
│  │ (read/     │  │ (dual-     │  │ (SQLite      │ │
│  │  write)    │  │  drive)    │  │  mirroring)  │ │
│  └────────────┘  └────────────┘  └──────────────┘ │
└──────────────────────┬──────────────────────────────┘
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
    ┌────────┐    ┌────────┐    ┌──────────┐
    │Primary │    │Backup  │    │Metadata  │
    │Drive   │    │Drive   │    │Cache     │
    │/photos │    │/backup │    │(optional)│
    └────────┘    └────────┘    └──────────┘
```

---

## Core Components

### 1. Frontend Layer (React + TypeScript)

#### Main Views
- **Library View**: Grid/list display of photos with filtering and sorting
- **Folder Browser**: Hierarchical folder navigation (sidebar)
- **Album Manager**: Create, edit, delete custom albums
- **Settings Panel**: Configure primary/backup drives, preferences
- **Search & Filter**: Advanced filtering UI (date range, tags, resolution, etc.)
- **Slideshow Mode**: Full-screen immersive viewing

#### State Management
- **React Query**: Manages server state (photos list, metadata, sync status)
- **Zustand Store**: Local UI state (selected items, filters, view mode, sidebar state)
- **Cache Strategy**: Local caching of photo metadata, invalidated on sync events

#### Component Library
- Use shadcn/ui for consistent, accessible components
- Custom hooks for recurring patterns (usePhotos, useSync, useFilters)
- Responsive design (works on desktop, adapts to different window sizes)

### 2. Backend Layer (Rust + Tauri)

#### File Operation Service
Handles all file system interactions with both drives

```rust
struct FileOperationService {
    primary_path: PathBuf,
    backup_path: PathBuf,
}

impl FileOperationService {
    // Scan directory recursively for photos
    async fn scan_directory(&self, path: &Path) -> Result<Vec<Photo>>;
    
    // List all supported image formats
    fn supported_formats() -> Vec<&'static str>;
    
    // Read EXIF and basic metadata
    async fn read_metadata(&self, path: &Path) -> Result<PhotoMetadata>;
    
    // Generate thumbnail (cached)
    async fn generate_thumbnail(&self, path: &Path, size: u32) -> Result<Bytes>;
}
```

#### Synchronization Engine
The core of dual-drive sync—keeps both drives in perfect lockstep

```rust
struct SyncEngine {
    primary: DriveManager,
    backup: DriveManager,
    db_primary: DatabaseConnection,
    db_backup: DatabaseConnection,
}

impl SyncEngine {
    // Execute operation on both drives atomically
    async fn execute_operation(&mut self, op: Operation) -> Result<()>;
    
    // Verify both drives have identical state
    async fn verify_sync_status(&self) -> Result<SyncStatus>;
    
    // Queue operations if backup is disconnected, flush when reconnected
    async fn handle_disconnected_backup(&mut self) -> Result<()>;
    
    // Recover backup to primary's state (in case of accidental deletion)
    async fn force_sync_backup_to_primary(&mut self) -> Result<()>;
}
```

#### Database Manager
SQLite instances on both drives, kept in sync

```rust
struct DatabaseManager {
    primary_db: SqlitePool,
    backup_db: SqlitePool,
}

// Schema includes:
// - photos (id, path, hash, date_taken, date_added, resolution, size)
// - albums (id, name, created_at)
// - photo_album_mapping (photo_id, album_id)
// - tags (id, name)
// - photo_tags (photo_id, tag_id)
// - sync_log (operation_id, timestamp, operation_type, status)
```

#### Operation Queue & Journal
For resilience and recovery

```rust
enum Operation {
    Move { from: PathBuf, to: PathBuf },
    Delete { path: PathBuf },
    Rename { path: PathBuf, new_name: String },
    AddToAlbum { photo_id: i64, album_id: i64 },
    RemoveFromAlbum { photo_id: i64, album_id: i64 },
    AddTag { photo_id: i64, tag_id: i64 },
}

// Each operation logged to sync journal before execution
// Enables rollback if one drive fails mid-operation
```

---

## Dual-Drive Synchronization Strategy

### Mental Model: Primary Branch & Replica

The primary drive is the source of truth. The backup drive replicates its exact state at all times.

### Operation Flow

1. **User initiates action** (rename, move, delete, etc.)
2. **Frontend sends command** to backend via Tauri IPC
3. **Backend validates** operation against current state
4. **Operation logged** to sync journal with unique ID
5. **Execute on primary drive** (file system + database)
6. **Mirror to backup drive** (file system + database)
7. **Both succeed or both fail** (atomic from user's perspective)
8. **Confirmation sent** to frontend, UI updates

### Handling Edge Cases

#### Backup Drive Disconnected
- Queue operations in memory and to sync journal
- When backup reconnects, flush all queued operations
- User sees status indicator (🔴 "Backup Offline") during disconnection

#### Accidental Deletion from Backup
- User deletes files directly from backup drive (outside app)
- Next sync verification detects mismatch
- App offers "Restore Backup" action—copies missing files back from primary
- Restores deleted metadata to backup DB from primary DB

#### Backup Drive Fails Mid-Operation
- Detect write failure to backup
- Roll back primary write (if not yet committed)
- Alert user, prompt to check backup drive connection
- Queue for retry when backup comes back online

#### Drive Full
- Check available space before operation
- Refuse operation if insufficient space on target
- Alert user with clear message about which drive is full

---

## Data Flow: Example - Renaming Photos in Bulk

```
User selects 10 photos → clicks "Rename" button
        ↓
Frontend sends: renamePhotos({ items: [id1, id2, ...], pattern: "Vacation_2024_%03d" })
        ↓
Backend validates: all files exist, new names don't conflict
        ↓
Log operation: { id: uuid, type: "bulk_rename", params: {...} }
        ↓
Execute on primary: rename files, update DB
        ↓
Mirror to backup: rename same files, update DB on backup drive
        ↓
Both succeed → emit sync_complete event to frontend
        ↓
Frontend updates UI: show new names, refresh photo grid
```

---

## Database Schema (SQLite)

```sql
-- Photos table
CREATE TABLE photos (
    id INTEGER PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    filename TEXT NOT NULL,
    file_hash TEXT NOT NULL,          -- SHA256 for duplicate detection
    file_size INTEGER,
    date_taken DATETIME,
    date_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    width INTEGER,
    height INTEGER,
    format TEXT                       -- JPEG, PNG, HEIC, etc.
);

-- Albums
CREATE TABLE albums (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Photo-Album mapping
CREATE TABLE photo_album (
    photo_id INTEGER,
    album_id INTEGER,
    PRIMARY KEY (photo_id, album_id),
    FOREIGN KEY (photo_id) REFERENCES photos(id),
    FOREIGN KEY (album_id) REFERENCES albums(id)
);

-- Tags
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

-- Photo-Tag mapping
CREATE TABLE photo_tag (
    photo_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (photo_id, tag_id),
    FOREIGN KEY (photo_id) REFERENCES photos(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

-- Sync journal for recovery
CREATE TABLE sync_operations (
    id TEXT PRIMARY KEY,
    operation_type TEXT NOT NULL,     -- move, delete, rename, etc.
    params JSON,
    status TEXT DEFAULT 'pending',    -- pending, completed, failed
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    error_message TEXT
);
```

---

## Performance Considerations

### Thumbnail Caching
- Generate thumbnails on first scan, cache in `.photovault/cache` folder on each drive
- Avoid regenerating on every app startup
- Clean cache on uninstall or user request

### Lazy Loading
- Load photo list in chunks (50-100 at a time)
- Use React Query's infinite query pattern for pagination
- Only load full metadata when user needs it

### Duplicate Detection
- Hash files incrementally (stream-based) for large files
- Cache hashes in DB to avoid re-hashing on subsequent scans
- Run detection as background task, don't block UI

### Sync Efficiency
- Batch operations where possible (multiple file moves in one transaction)
- Write to DB in transactions, not individual inserts
- Only sync changes, not full re-scan of drives

---

## Tauri Commands (IPC Interface)

```rust
#[tauri::command]
async fn scan_library(primary_path: String, backup_path: String) -> Result<Vec<Photo>>;

#[tauri::command]
async fn move_photos(photo_ids: Vec<i64>, target_folder: String) -> Result<()>;

#[tauri::command]
async fn rename_photo(photo_id: i64, new_name: String) -> Result<()>;

#[tauri::command]
async fn bulk_rename(photo_ids: Vec<i64>, pattern: String) -> Result<()>;

#[tauri::command]
async fn create_album(name: String) -> Result<Album>;

#[tauri::command]
async fn add_to_album(photo_ids: Vec<i64>, album_id: i64) -> Result<()>;

#[tauri::command]
async fn find_duplicates(threshold: f32) -> Result<Vec<DuplicateGroup>>;

#[tauri::command]
async fn delete_duplicates(duplicate_ids: Vec<i64>) -> Result<()>;

#[tauri::command]
async fn verify_sync_status() -> Result<SyncStatus>;

#[tauri::command]
async fn restore_backup() -> Result<()>;

#[tauri::command]
async fn get_filter_options() -> Result<FilterOptions>;

#[tauri::command]
async fn filter_photos(filters: FilterCriteria) -> Result<Vec<Photo>>;

#[tauri::command]
async fn add_tag(photo_id: i64, tag_name: String) -> Result<()>;
```

---

## Build & Deployment

### Development
```bash
# Frontend dev server
npm run dev

# Backend + Tauri in dev mode
npm run tauri dev
```

### Production Build
```bash
# Creates platform-specific binaries
npm run tauri build

# Outputs to src-tauri/target/release/bundle/
# - .msi (Windows)
# - .dmg + .app (macOS)
# - .AppImage + .deb (Linux)
```

### Code Signing & Distribution
- Sign Windows builds with code certificate
- Notarize macOS builds with Apple Developer account
- Host on website with auto-update support via Tauri's updater

---

## Future Architecture Considerations

- **Cloud Integration**: Add optional backup to Nextcloud/Synology as tertiary backup
- **Multi-Device Sync**: Optional cloud relay for syncing across devices (separate toggle)
- **Plugin System**: Allow third-party integrations for advanced editing/organization
- **GPU Acceleration**: Offload thumbnail generation to GPU for massive libraries

---

## Development Philosophy

- **Local-First**: All operations work offline; cloud is optional
- **Atomic Operations**: All-or-nothing behavior—no partial syncs
- **User Control**: Users choose how to organize; no hidden AI decisions
- **Performance**: Lightweight, responsive, snappy
- **Transparency**: Users see what's happening (clear status, error messages)
