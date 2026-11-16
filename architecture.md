# Architecture: VeloMotion Clone (Rust Fullstack & Client-Side NLE)

## Stack

| layer | choice | why |
| --- | --- | --- |
| web frontend | Leptos (Rust WASM) + Tailwind CSS | High performance, memory-safe client-side rendering and NLE computation compiled to WebAssembly, static deployment on GitHub Pages. |
| backend | Rust (Vercel Serverless Functions in `api/`) | Lightweight, fast execution runtime for validating and persisting project JSON state without media binary overhead. |
| database | Supabase (PostgreSQL) | Managed relational database with Row Level Security (RLS) for user projects and metadata storage. |
| auth | Supabase Auth (JWT) | Secure session management integrated directly with PostgreSQL RLS. |
| hosting | GitHub Pages (Frontend) + Vercel (Backend API) | Free, performant static hosting for WASM bundle and serverless edge compute for JSON persistence. |

---

## Folder Structure (Strict Monorepo / Separation)

```text
velomotion-studio/
├── frontend/                     # Leptos WASM Frontend (Deployed to GitHub Pages)
│   ├── Cargo.toml                # Leptos, wasm-bindgen, web-sys, reqwasm, serde
│   ├── index.html                # Tailwind CSS CDN / compiled CSS entry
│   ├── tailwind.config.js
│   └── src/
│       ├── main.rs               # App entry point & router setup (2 Pages)
│       ├── views/
│       │   ├── dashboard.rs      # PAGE 1: Project Dashboard (Recent Projects & Create button)
│       │   └── editor.rs         # PAGE 2: Video Editor Workspace (5 Mandatory Zones)
│       ├── components/
│       │   ├── canvas_preview.rs # Zone A: requestAnimationFrame loop canvas renderer
│       │   ├── timeline_nles.rs  # Zone B: Horizontal virtual timeline NLE UI
│       │   ├── media_import.rs   # Zone C: File System Access API local loader
│       │   ├── toolbar.rs        # Zone D: Trim In/Out, Split, Delete with Ripple Edit
│       │   └── export_engine.rs  # Zone E: Local FFmpeg.wasm fast stream-copy exporter
│       ├── state/                # Application state store (Timeline struct, active project)
│       └── utils/                # FFmpeg WASM bindings & canvas helper utilities
│
├── api/                          # Rust Vercel Serverless Functions (Backend API)
│   ├── Cargo.toml                # Shared serde, postgres/supabase client
│   ├── save_project/             # POST /api/save_project (Saves JSON timeline state)
│   │   └── main.rs
│   └── load_project/             # GET /api/load_project?id=... (Loads JSON timeline state)
│       └── main.rs
│
├── replica/                      # Recon & Architecture documentation
│   ├── architecture.md           # This document
│   └── schema.sql                # Supabase PostgreSQL schema definition
└── README.md
```

---

## Routing & Page Specifications (Strict 2-Page SPA)

### PAGE 1: Project Dashboard (`/`)
- **Purpose**: Displays the list of 'Recent Video Projects' fetched from the Supabase database via Vercel API (`GET /api/load_project`).
- **UI Elements**:
  - Grid/List of existing projects with thumbnail previews, titles, and last modified timestamps.
  - **'Create New Video Project'** primary action button which instantiates a new project record, saves initial empty JSON state, and instantly routes the user to **Page 2** (`/editor?id=...`).

### PAGE 2: Video Editor Workspace (`/editor`)
The core editing environment containing exactly 5 mandatory functional zones:

1. **Zone A: Video Canvas Preview**
   - **Mechanism**: High-performance HTML5 `<canvas>` rendering loop driven by `requestAnimationFrame` (bypassing Leptos reactive overhead for 60fps smoothness).
   - **Features**: Real-time multi-track compositing, playhead scrubbing, zoom/pan controls, and visual overlay preview.
2. **Zone B: Video Timeline NLE**
   - **Mechanism**: Horizontal timeline UI managing a virtual `Timeline` state array (`Vec<Track>` containing `Vec<Clip>`).
   - **Features**: Non-destructive editing (clips reference local object URLs / File Handles without modifying original source files), track stacking (video, audio, text overlays), and snapping.
3. **Zone C: Media Import**
   - **Mechanism**: Utilizes the browser's **File System Access API** (`window.showOpenFilePicker`) and HTML File inputs for local file loading.
   - **Constraint**: **Zero backend upload**. Media files remain 100% local in browser memory/IndexedDB object URLs.
4. **Zone D: Editing Toolbar**
   - **Operations**:
     - **Trim In / Trim Out**: Adjust start/end sample offsets of active clips.
     - **Split**: Cut a clip at the current playhead position into two discrete virtual clips.
     - **Delete with Ripple Edit**: Remove a clip and automatically shift subsequent timeline clips leftward to close gaps.
5. **Zone E: Export Engine**
   - **Mechanism**: Integrated **FFmpeg.wasm** running entirely in a client-side Web Worker.
   - **Flow**: Compiles the virtual timeline JSON state into FFmpeg filtergraph commands, executes fast stream-copy/transmuxing locally, and triggers browser file download for the final `.mp4` rendered file.

---

## Database Schema (`schema.sql`)

```sql
create table projects (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null,
  title text not null default 'Untitled Project',
  timeline_json jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index idx_projects_user_id on projects (user_id);
create index idx_projects_updated_at on projects (updated_at desc);

-- Enable Row Level Security (RLS)
alter table projects enable row level security;

create policy "Users can CRUD their own projects"
  on projects for all
  using (auth.uid() = user_id)
  with check (auth.uid() = user_id);
```

---

## Vercel Serverless API Endpoints (Rust)

Because Vercel serverless functions have strict payload size limits and execution timeouts, **no media binaries (MP4/WAV) ever pass through the backend**. The backend is strictly a JSON state store.

### 1. `POST /api/save_project`
- **Purpose**: Validates and persists the client's timeline JSON state to Supabase.
- **Request Body (JSON)**:
  ```json
  {
    "id": "uuid-optional-if-new",
    "title": "My VeloMotion Edit",
    "timeline_json": {
      "version": "1.0",
      "duration": 30.5,
      "tracks": [...]
    }
  }
  ```
- **Response**: `{ "success": true, "id": "uuid", "updated_at": "timestamp" }`

### 2. `GET /api/load_project`
- **Purpose**: Retrieves project metadata and timeline JSON state for Dashboard listing or Editor loading.
- **Query Params**: `?id=uuid` (or `?user_id=uuid` for dashboard listing).
- **Response**: Project JSON record.

---

## The Parts That Bite

1. **WASM Memory & Large State**: Managing large timeline objects in Rust WASM requires efficient serialization (`serde`) and avoiding clone overhead in hot rendering loops.
2. **FFmpeg.wasm Threading**: Requires `SharedArrayBuffer` headers (`Cross-Origin-Opener-Policy: same-origin`, `Cross-Origin-Embedder-Policy: require-corp`) enabled on GitHub Pages / hosting provider.
3. **File Handle Persistence**: Browser security revokes File System Access API handles across page reloads; fallback to IndexedDB blob storage for imported media assets is mandatory.
4. **requestAnimationFrame Synchronization**: Decoupling the high-frequency canvas render loop from Leptos reactive state signals to prevent UI lag.

---

## Build Order

1. **Vertical Slice**: Leptos project skeleton with Page 1 (Dashboard list) -> Page 2 loading blank editor with Local File import, Canvas Preview (`requestAnimationFrame`), and basic timeline array state.
2. **Must-Haves**:
   - Vercel Rust API endpoints (`/api/save_project`, `/api/load_project`) connected to Supabase PostgreSQL.
   - NLE editing toolbar operations (Trim, Split, Ripple Delete).
   - FFmpeg.wasm local export engine generating downloadable MP4.
3. **Should-Haves**: Multi-track layer stacking, undo/redo state history, and snap-to-grid timeline interactions.
4. **Refinements**: Tailwind CSS styling matching VeloMotion's dark-mode pro editing interface aesthetics.
