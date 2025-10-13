# PhotoVault - Implementation Plan

This guide breaks down PhotoVault development into manageable phases with clear deliverables, milestones, and testing strategies.

**Estimated Timeline**: 16-20 weeks (vibe coding pace, working solo)

---

## Phase 0: Project Setup (Week 1)

Get the foundation solid so you can build fast later.

### Deliverables
- [ ] Tauri + React project scaffolded
- [ ] TypeScript configured
- [ ] Tailwind CSS + shadcn/ui set up
- [ ] Basic project structure established
- [ ] Development and build pipelines working

### Tasks

1. **Initialize Tauri Project**
   ```bash
   npm create tauri-app@latest photovault
   # Choose: React, TypeScript, Vite
   ```

2. **Set Up Frontend Stack**
   - Install Tailwind CSS: `npm install -D tailwindcss postcss autoprefixer`
   - Set up shadcn/ui: `npx shadcn-ui@latest init`
   - Configure Vite for fast HMR

3. **Configure Rust Backend**
   - Verify Tauri core dependencies (already in place)
   - Add to `Cargo.toml`:
     ```toml
     tokio = { version = "1", features = ["full"] }
     sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
     walkdir = "2"
     image = "0.24"
     sha2 = "0.10"
     serde = { version = "1.0", features = ["derive"] }
     serde_json = "1.0"
     ```

4. **Project Structure**
   ```
   photovault/
   ├── src/
   │   ├── components/        (React components)
   │   ├── hooks/            (Custom React hooks)
   │   ├── stores/           (Zustand state)
   │   ├── types/            (TypeScript types)
   │   ├── pages/            (Main views)
   │   └── App.tsx
   ├── src-tauri/
   │   ├── src/
   │   │   ├── commands/     (Tauri command handlers)
   │   │   ├── services/     (Business logic)
   │   │   ├── models/       (Data structures)
   │   │   ├── db/           (Database management)
   │   │   └── main.rs       (Entry point)
   │   └── Cargo.toml
   └── [config files]
   ```

5. **Test Setup**
   - Configure Jest for React components
   - Set up Rust tests (built-in via `#[cfg(test)]`)

6. **Version Control**
   - Initialize git repo
   - Create `.gitignore` for Tauri/Node
   - Initial commit

### Success Criteria
- `npm run dev` starts frontend + backend
- `npm run tauri dev` launches desktop app window
- No errors on startup

---

## Phase 1: Core File Scanning & Display (Weeks 2-3)

Get photos from disk into the UI. This is foundational—you need this working before sync.

### Deliverables
- [ ] Scan primary drive for photos
- [ ] Display photos in grid view
- [ ] Basic filtering (by file type, date range)
- [ ] Folder navigation working
- [ ] Photo metadata extracted

### Backend Tasks

1. **File Operation Service** (`src-tauri/src/services/file_ops.rs`)
   ```rust
   pub struct FileOperationService {
       primary_path: PathBuf,
   }

   impl FileOperationService {
       pub async fn scan_directory(&self, path: &Path) -> Result<Vec<Photo>>;
       pub fn get_supported_formats() -> Vec<&'static str>;
       pub async fn read_metadata(&self, path: &Path) -> Result<PhotoMetadata>;
   }
   ```

2. **Photo Model** (`src-tauri/src/models/photo.rs`)
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct Photo {
       pub id: i64,
       pub path: String,
       pub filename: String,
       pub file_size: u64,
       pub date_taken: Option<DateTime<Utc>>,
       pub width: u32,
       pub height: u32,
       pub format: String,
   }
   ```

3. **Database Setup** (`src-tauri/src/db/mod.rs`)
   - Initialize SQLite on primary drive
   - Create schema (photos table at minimum for Phase 1)
   - Write migrations

4. **Tauri Commands** (`src-tauri/src/commands/mod.rs`)
   ```rust
   #[tauri::command]
   async fn scan_library(primary_path: String) -> Result<Vec<Photo>>;
   
   #[tauri::command]
   async fn get_photos(limit: i64, offset: i64) -> Result<Vec<Photo>>;
   ```

### Frontend Tasks

1. **Gallery Component** (`src/components/Gallery.tsx`)
   - Grid display using shadcn components
   - Lazy loading photos (React Query infinite scroll)
   - Image placeholder while loading

2. **Sidebar Navigation** (`src/components/Sidebar.tsx`)
   - Folder tree view
   - Current folder display
   - Navigation between folders

3. **Main App Layout** (`src/pages/Library.tsx`)
   - Combine gallery + sidebar
   - Basic responsive layout

4. **Store Setup** (`src/stores/photoStore.ts`)
   - Zustand store for selected folder, view settings
   - React Query for photo list management

### Testing
- Backend: Unit tests for metadata extraction, path handling
- Frontend: Component renders with mock data
- Integration: Scan a test folder, verify photos appear in UI

### Success Criteria
- Scan a folder with 50 photos in ~2 seconds
- Photos display in grid with thumbnails
- No database errors
- Can navigate folders

---

## Phase 2: Database & Dual Drive Setup (Weeks 4-5)

Prepare the infrastructure for the killer feature. This is critical.

### Deliverables
- [ ] SQLite database on primary drive
- [ ] Database mirrored to backup drive
- [ ] Drive selection UI working
- [ ] Sync status detection
- [ ] Sync verification endpoint

### Backend Tasks

1. **Database Manager** (`src-tauri/src/db/manager.rs`)
   ```rust
   pub struct DatabaseManager {
       primary_db: SqlitePool,
       backup_db: Option<SqlitePool>,
   }

   impl DatabaseManager {
       pub async fn initialize(primary_path: PathBuf, backup_path: Option<PathBuf>) -> Result<Self>;
       pub async fn execute_on_both(&mut self, query: &str, params: &[&str]) -> Result<()>;
   }
   ```

2. **Complete Database Schema** (`src-tauri/src/db/schema.sql`)
   - photos, albums, photo_album, tags, photo_tags, sync_operations tables

3. **Migration System**
   - Use sqlx migrations directory
   - `migrations/20250101_initial.sql`

4. **Sync Status Service** (`src-tauri/src/services/sync_status.rs`)
   ```rust
   pub struct SyncStatus {
       pub primary_connected: bool,
       pub backup_connected: bool,
       pub last_sync: DateTime<Utc>,
       pub is_in_sync: bool,
       pub pending_operations: u32,
   }

   pub async fn verify_sync_status() -> Result<SyncStatus>;
   ```

5. **Drive Configuration** (`src-tauri/src/services/config.rs`)
   - Store drive paths in config file (`~/.photovault/config.json`)
   - Validate drives exist and are readable/writable

### Frontend Tasks

1. **Drive Selection Modal** (`src/components/DriveSetupModal.tsx`)
   - File picker for primary drive
   - File picker for backup drive
   - Validation and error display

2. **Settings Panel** (`src/pages/Settings.tsx`)
   - Display current drives
   - Change drives
   - Verify sync button
   - Backup status indicator

3. **Status Bar** (`src/components/StatusBar.tsx`)
   - Show backup status (connected/disconnected)
   - Show pending sync operations count
   - Quick access to verify sync

### Tauri Commands
```rust
#[tauri::command]
async fn set_drive_paths(primary: String, backup: String) -> Result<()>;

#[tauri::command]
async fn verify_sync_status() -> Result<SyncStatus>;

#[tauri::command]
async fn get_config() -> Result<AppConfig>;
```

### Testing
- Verify both databases exist after setup
- Verify schema matches on both drives
- Check sync detection with missing backup
- Validate drive paths

### Success Criteria
- Can select and configure primary + backup drives
- Status bar shows correct backup status
- Both databases exist and are synced
- Sync verification completes without error

---

## Phase 3: Sync Engine (Weeks 6-8)

Build the core synchronization logic. This is the heart of the app.

### Deliverables
- [ ] Operation logging system
- [ ] Atomic dual-drive writes
- [ ] Sync engine command execution
- [ ] Error handling & rollback
- [ ] Queue system for disconnected backups

### Backend Tasks

1. **Operation Types** (`src-tauri/src/models/operation.rs`)
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub enum Operation {
       Move { from: PathBuf, to: PathBuf },
       Delete { path: PathBuf },
       Rename { path: PathBuf, new_name: String },
       CreateAlbum { name: String },
       AddToAlbum { photo_id: i64, album_id: i64 },
       AddTag { photo_id: i64, tag_name: String },
   }
   ```

2. **Sync Engine** (`src-tauri/src/services/sync_engine.rs`)
   ```rust
   pub struct SyncEngine {
       primary_db: SqlitePool,
       backup_db: Option<SqlitePool>,
       operation_queue: Vec<Operation>,
   }

   impl SyncEngine {
       pub async fn execute_operation(&mut self, op: Operation) -> Result<()>;
       pub async fn execute_on_both(&mut self, op: &Operation) -> Result<()>;
       pub async fn handle_backup_disconnected(&mut self, op: Operation) -> Result<()>;
       pub async fn flush_queue(&mut self) -> Result<()>;
   }
   ```

3. **Operation Journaling**
   - Log all operations to `sync_operations` table
   - Track status: pending, completed, failed
   - Enable recovery and debugging

4. **Error Handling**
   - If operation fails on primary, don't execute on backup
   - If operation fails on backup but succeeded on primary, queue for retry
   - Clear error reporting to user

5. **Disconnection Handling**
   - Detect when backup drive is unreachable
   - Queue operations in memory + sync journal
   - On reconnect, flush queue
   - Warn user if queue grows too large

### Frontend Tasks

1. **Operation Feedback UI**
   - Toast notifications for sync events
   - Error display with recovery options
   - Progress indicator during long operations

2. **Queue Status Display** (`src/components/SyncQueue.tsx`)
   - Show pending operations count
   - Show sync in progress
   - Manual retry button if failed

### Tauri Commands
```rust
#[tauri::command]
async fn move_photos(photo_ids: Vec<i64>, target_path: String) -> Result<()>;

#[tauri::command]
async fn delete_photos(photo_ids: Vec<i64>) -> Result<()>;

#[tauri::command]
async fn rename_photo(photo_id: i64, new_name: String) -> Result<()>;

#[tauri::command]
async fn get_sync_queue_status() -> Result<QueueStatus>;
```

### Testing
- Unit tests for each operation type
- Test with backup disconnected (should queue)
- Test backup reconnection (should flush queue)
- Test failure on one drive (should not corrupt the other)
- Integration: perform 10 operations, verify both drives match

### Success Criteria
- Operations execute on both drives atomically
- Backup disconnection doesn't crash app
- Queue flushes on reconnect
- All operations logged for debugging
- No data loss or corruption in edge cases

---

## Phase 4: Album Management & Organization (Weeks 9-10)

Core organization features.

### Deliverables
- [ ] Create/edit/delete albums
- [ ] Add photos to albums
- [ ] Album view in UI
- [ ] Bulk operations (move, add to album)
- [ ] All synced to both drives

### Backend Tasks

1. **Album Service** (`src-tauri/src/services/album.rs`)
   ```rust
   pub struct AlbumService;

   impl AlbumService {
       pub async fn create_album(name: String) -> Result<Album>;
       pub async fn add_photos_to_album(photo_ids: Vec<i64>, album_id: i64) -> Result<()>;
       pub async fn remove_photos_from_album(photo_ids: Vec<i64>, album_id: i64) -> Result<()>;
       pub async fn delete_album(album_id: i64) -> Result<()>;
   }
   ```

2. **Album Operations** via Sync Engine
   - All album operations go through sync engine
   - Executed on both drives' databases

### Frontend Tasks

1. **Album Manager** (`src/components/AlbumManager.tsx`)
   - Create album modal
   - Album list sidebar
   - Album view (filtered photos)

2. **Bulk Operations UI** (`src/components/BulkActions.tsx`)
   - Checkbox selection
   - Bulk move to album
   - Bulk delete
   - Bulk rename

3. **Album Display**
   - Show album name and photo count
   - Quick access from sidebar
   - Album details view

### Tauri Commands
```rust
#[tauri::command]
async fn create_album(name: String) -> Result<Album>;

#[tauri::command]
async fn add_photos_to_album(photo_ids: Vec<i64>, album_id: i64) -> Result<()>;

#[tauri::command]
async fn get_albums() -> Result<Vec<Album>>;

#[tauri::command]
async fn delete_album(album_id: i64) -> Result<()>;
```

### Testing
- Create album, verify on both drives
- Add photos, verify sync
- Delete album, verify cleanup
- Bulk operations with many photos

### Success Criteria
- Create albums and add photos
- Changes sync to backup instantly
- Delete albums safely
- Bulk operations work smoothly
- No orphaned data

---

## Phase 5: Tagging, Filtering & Search (Weeks 11-12)

Discovery and organization.

### Deliverables
- [ ] Tag system working
- [ ] Advanced filtering UI
- [ ] Filter by date, resolution, size, tags
- [ ] Search functionality
- [ ] Synced across both drives

### Backend Tasks

1. **Tag Service** (`src-tauri/src/services/tags.rs`)
   ```rust
   pub struct TagService;

   impl TagService {
       pub async fn add_tag(photo_id: i64, tag_name: String) -> Result<()>;
       pub async fn remove_tag(photo_id: i64, tag_id: i64) -> Result<()>;
       pub async fn get_photo_tags(photo_id: i64) -> Result<Vec<Tag>>;
   }
   ```

2. **Filter Service** (`src-tauri/src/services/filter.rs`)
   ```rust
   #[derive(Debug, Clone)]
   pub struct FilterCriteria {
       pub date_from: Option<DateTime<Utc>>,
       pub date_to: Option<DateTime<Utc>>,
       pub min_width: Option<u32>,
       pub min_height: Option<u32>,
       pub tags: Vec<i64>,
       pub albums: Vec<i64>,
       pub query: Option<String>,
   }

   pub async fn filter_photos(criteria: FilterCriteria) -> Result<Vec<Photo>>;
   ```

3. **Database Indexing**
   - Add indexes on date_taken, width, height, file_size for fast queries

### Frontend Tasks

1. **Filter Panel** (`src/components/FilterPanel.tsx`)
   - Date range picker
   - Resolution filter
   - Size range
   - Tag selector
   - Album selector

2. **Search Box** (`src/components/SearchBox.tsx`)
   - Real-time filename search
   - Tag search
   - Album search

3. **Filter Chips** (`src/components/FilterChips.tsx`)
   - Show active filters
   - Remove filters individually
   - Clear all filters

### Tauri Commands
```rust
#[tauri::command]
async fn filter_photos(criteria: FilterCriteria) -> Result<Vec<Photo>>;

#[tauri::command]
async fn add_tag(photo_id: i64, tag_name: String) -> Result<()>;

#[tauri::command]
async fn get_all_tags() -> Result<Vec<Tag>>;

#[tauri::command]
async fn search_photos(query: String) -> Result<Vec<Photo>>;
```

### Testing
- Filter by each criterion individually
- Combine multiple filters
- Search performance with 10k photos
- Tag sync across drives

### Success Criteria
- Filtering is fast even with large libraries
- All filters work individually and combined
- Search returns relevant results
- Tags sync correctly

---

## Phase 6: Duplicate Detection & Removal (Week 13)

Cleanup feature that's genuinely useful.

### Deliverables
- [ ] Hash-based duplicate detection
- [ ] Duplicate grouping UI
- [ ] Safe deletion of duplicates
- [ ] Sync deletion across drives

### Backend Tasks

1. **Duplicate Detection Service** (`src-tauri/src/services/duplicate.rs`)
   ```rust
   pub struct DuplicateDetector;

   impl DuplicateDetector {
       pub async fn find_duplicates(threshold: f32) -> Result<Vec<DuplicateGroup>>;
       pub async fn hash_file(path: &Path) -> Result<String>;
       pub async fn cache_hash(photo_id: i64, hash: String) -> Result<()>;
   }
   ```

2. **Hashing Strategy**
   - Use SHA256
   - Cache hashes in DB (file_hash column)
   - Lazy compute on first scan
   - Stream large files to avoid memory bloat

3. **Duplicate Groups**
   ```rust
   pub struct DuplicateGroup {
       pub hash: String,
       pub photos: Vec<Photo>,
       pub size: u64,
   }
   ```

### Frontend Tasks

1. **Duplicate Inspector** (`src/components/DuplicateInspector.tsx`)
   - Show duplicate groups
   - Side-by-side photo comparison
   - Select which duplicates to keep
   - Show space savings

2. **Deletion UI**
   - Preview what will be deleted
   - Confirmation dialog
   - Progress indicator

### Tauri Commands
```rust
#[tauri::command]
async fn find_duplicates() -> Result<Vec<DuplicateGroup>>;

#[tauri::command]
async fn delete_duplicates(photo_ids: Vec<i64>) -> Result<i64>; // returns space freed
```

### Testing
- Create duplicate test set
- Verify all duplicates found
- Verify deletion works on both drives
- Verify metadata cleaned up

### Success Criteria
- All duplicates correctly identified
- Safe deletion without data loss
- Changes synced to backup
- Space savings displayed accurately

---

## Phase 7: Bulk Rename & Metadata Editing (Week 14)

Powerful organizational tool.

### Deliverables
- [ ] Bulk rename with patterns
- [ ] Rename templates (date-based, sequential, etc.)
- [ ] Metadata editing UI
- [ ] Sync all changes

### Backend Tasks

1. **Rename Service** (`src-tauri/src/services/rename.rs`)
   ```rust
   pub struct RenameService;

   impl RenameService {
       pub async fn bulk_rename(photo_ids: Vec<i64>, pattern: String) -> Result<()>;
       // Pattern: "Vacation_{date}_{index}"
   }
   ```

2. **Pattern Variables**
   - `{date}`: Photo date taken
   - `{index}`: Sequential number
   - `{album}`: Album name
   - `{original}`: Original filename

### Frontend Tasks

1. **Bulk Rename UI** (`src/components/BulkRenameModal.tsx`)
   - Pattern input with template suggestions
   - Preview of results before executing
   - Confirmation

### Tauri Commands
```rust
#[tauri::command]
async fn bulk_rename(photo_ids: Vec<i64>, pattern: String) -> Result<Vec<RenameResult>>;

#[tauri::command]
async fn preview_bulk_rename(photo_ids: Vec<i64>, pattern: String) -> Result<Vec<RenamePreview>>;
```

### Testing
- Test each pattern variable
- Test rename with special characters
- Verify no filename conflicts
- Test sync of renamed files

### Success Criteria
- Bulk rename works with multiple photos
- Preview shows correct results
- No filename collisions
- Changes synced instantly

---

## Phase 8: Slideshow & Basic Viewing (Week 15)

Nice-to-have that's quick to implement.

### Deliverables
- [ ] Full-screen slideshow mode
- [ ] Auto-advance with configurable speed
- [ ] Manual navigation
- [ ] Keyboard controls

### Frontend Tasks

1. **Slideshow Component** (`src/components/Slideshow.tsx`)
   - Full-screen view
   - Auto-advance timer
   - Keyboard controls (left/right, esc)
   - Photo info overlay (optional)

2. **Slideshow Trigger**
   - Add button to gallery
   - Keyboard shortcut (S key)

### Testing
- Test slideshow timing
- Test keyboard controls
- Test with filtered photo set
- Test exit handling

### Success Criteria
- Slideshow plays smoothly
- Controls responsive
- Timer works accurately

---

## Phase 9: Restore Backup Feature (Week 16)

Critical safety feature for accidental deletion recovery.

### Deliverables
- [ ] Detect when backup differs from primary
- [ ] One-click restore option
- [ ] Preserve primary state
- [ ] Show what will be restored

### Backend Tasks

1. **Restore Service** (`src-tauri/src/services/restore.rs`)
   ```rust
   pub struct RestoreService;

   impl RestoreService {
       pub async fn detect_differences() -> Result<RestoreSummary>;
       pub async fn restore_backup_to_primary() -> Result<()>;
   }
   ```

2. **Restore Logic**
   - Compare file hashes
   - Compare DB states
   - Offer preview before restoring
   - Log restore operation

### Frontend Tasks

1. **Restore UI** (`src/components/RestoreBackupModal.tsx`)
   - Show what's missing/different
   - File count and size
   - Confirmation with warnings
   - Progress indicator

### Tauri Commands
```rust
#[tauri::command]
async fn detect_backup_differences() -> Result<RestoreSummary>;

#[tauri::command]
async fn restore_backup_to_primary() -> Result<()>;
```

### Testing
- Manually delete files from primary
- Run restore, verify they come back
- Verify secondary stays intact
- Test with many files

### Success Criteria
- Accidental deletion recoverable
- No data loss during restore
- Clear user feedback during process

---

## Phase 10: Settings, Config & Polish (Week 17)

User experience refinement and configuration.

### Deliverables
- [ ] Settings panel fully functional
- [ ] Persistence of user preferences
- [ ] App startup optimization
- [ ] Error handling polish
- [ ] Status messages clarity

### Frontend Tasks

1. **Settings Pages** (`src/pages/Settings.tsx`)
   - App preferences (theme, startup behavior)
   - Drive configuration
   - About/version info
   - Reset/clear cache options

2. **Error Handling UI**
   - Better error messages
   - Actionable suggestions for failures
   - Support contact info

3. **Polish**
   - Loading states
   - Keyboard shortcuts documentation
   - App tour/onboarding (optional)

### Backend Tasks

1. **Config Management** (`src-tauri/src/services/config.rs`)
   - Save/load user preferences
   - Validate config on startup

### Success Criteria
- Settings persist across restarts
- All features configurable
- Clear error messages
- Smooth startup

---

## Phase 11: Testing & Quality Assurance (Week 18)

Comprehensive testing before release.

### Deliverables
- [ ] Unit tests for all services
- [ ] Integration tests for sync workflow
- [ ] UI component tests
- [ ] Manual QA checklist completed
- [ ] Performance benchmarks

### Backend Testing

1. **Unit Tests**
   - File operations (scan, read metadata)
   - Database queries
   - Sync logic
   - Filter logic
   - Duplicate detection

2. **Integration Tests**
   - Full sync workflow (10+ operations)
   - Error recovery scenarios
   - Backup disconnection/reconnection
   - Massive file operations (1000+ photos)

3. **Performance Benchmarks**
   - Scan 10k photos: < 30 seconds
   - Filter 10k photos: < 1 second
   - Find duplicates in 10k photos: < 60 seconds
   - Each operation sync: < 2 seconds

### Frontend Testing

1. **Component Tests**
   - Gallery rendering with mock data
   - Filter application
   - Modal interactions
   - Error display

2. **Manual QA**
   - Full workflow test
   - Edge case testing
   - UI responsiveness check
   - Performance testing with real library

### Success Criteria
- 80%+ test coverage on backend
- 50%+ test coverage on frontend
- All performance benchmarks met
- QA checklist 100% complete

---

## Phase 12: Build, Sign & Release (Week 19-20)

Prepare for distribution.

### Deliverables
- [ ] Production builds for all platforms
- [ ] Code signing certificates obtained
- [ ] Updater mechanism tested
- [ ] Documentation finalized
- [ ] Website/download page ready

### Build Tasks

1. **Platform-Specific Builds**
   ```bash
   npm run tauri build
   # Generates: .msi (Windows), .dmg (macOS), .AppImage (Linux)
   ```

2. **Code Signing**
   - Windows: EV code certificate
   - macOS: Apple Developer account + notarization
   - Linux: PGP signature (optional)

3. **Auto-Update Setup**
   - Configure Tauri updater
   - Host updates on server
   - Version management

### Documentation

1. **User Documentation**
   - Quick start guide
   - FAQ
   - Troubleshooting
   - Keyboard shortcuts

2. **Developer Documentation**
   - For future maintenance/contributors
   - Build instructions
   - API reference
   - Architecture overview (you already have this!)

### Website/Installer

1. **Download Page**
   - Platform detection
   - Version info
   - Changelog
   - Instructions

2. **Installer Testing**
   - Test on clean system
   - Verify first-run experience
   - Check uninstall cleanup

### Success Criteria
- All platforms build without errors
- Code signing works (no warnings)
- Auto-updater tested end-to-end
- Installation smooth on fresh system

---

## Daily Workflow Checklist

For keeping momentum and tracking progress:

### Daily (15 min standup with yourself)
- [ ] What did I ship yesterday?
- [ ] What am I shipping today?
- [ ] Any blockers?
- [ ] Update progress on current phase

### Weekly (30 min reflection)
- [ ] Did I hit my phase goals?
- [ ] How's performance?
- [ ] Any technical debt to address?
- [ ] Adjust next week's plan if needed

### Phase Completion (1 hour)
- [ ] All deliverables done?
- [ ] Tests passing?
- [ ] Known issues documented?
- [ ] Ready for next phase?

---

## Risk & Mitigation

### Technical Risks

**Risk**: Sync engine has edge case bugs
- *Mitigation*: Extensive testing in Phase 3, operation journaling, rollback capability

**Risk**: Performance degrades with 100k+ photos
- *Mitigation*: Pagination, indexing, lazy loading from start

**Risk**: Database corruption on backup drive
- *Mitigation*: Atomic operations, transaction support, regular integrity checks

### Project Risks

**Risk**: Scope creep (adding features mid-phase)
- *Mitigation*: Strict phase boundaries, defer non-essential features to "Future Enhancements"

**Risk**: Motivation dips on solo project
- *Mitigation*: Weekly shipped features, celebrate phase completions, stay focused on core

**Risk**: Platform-specific issues discovered late
- *Mitigation*: Test on all three platforms starting in Phase 1

---

## Phase Dependencies

Phases are mostly sequential, but some can overlap:

- Phases 0-3: Sequential (foundation)
- Phases 4-8: Can be reordered based on priority
- Phases 9-10: Before testing
- Phases 11-12: Must be last

## Time Buffer

Built-in 10-20% time buffer for:
- Unexpected bugs
- Platform-specific issues
- Design iterations
- Context switching overhead

Adjust phase durations based on your actual vibe coding pace.

---

## Post-Launch Roadmap

After 1.0 release:

- **1.1**: Cloud backup integration (Nextcloud)
- **1.2**: Mobile companion app
- **1.3**: Advanced search with ML
- **2.0**: Multi-device sync (optional cloud)

Keep 1.0 focused on the core promise: **organize photos locally, sync two drives perfectly**.
