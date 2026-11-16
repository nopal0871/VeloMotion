# Test Plan: VeloMotion Clone (Rust Leptos Client NLE & Vercel API)

## Overview
This test plan covers the 2-page SPA architecture of the VeloMotion clone, with special emphasis on the 5 zones in Page 2 (Video Editor Workspace) and client-side NLE operations.

---

## Flow 1: Project Dashboard (Page 1)
- **F01-H1 (Happy Path)**: Load `/` and verify list of recent video projects fetched from Supabase via Vercel API. Click "Create New Video Project" and verify smooth navigation to `/editor/{id}`.
- **F01-E1 (Empty State)**: Render dashboard when zero projects exist. Verify placeholder message and prominent create button.
- **F01-N1 (API Error)**: Simulate Vercel API `/api/load_project` failure. Verify error banner and retry option.

---

## Flow 2: Video Editor Workspace - 5 Zones (Page 2)
### Zone A: Video Canvas Preview (`requestAnimationFrame` Loop)
- **F02-A1**: Verify 60fps render loop runs independently of Leptos UI signals.
- **F02-A2**: Verify playhead scrubbing updates canvas frame accurately without UI blocking.

### Zone B: Video Timeline NLE (Virtual `Vec<VideoClip>` State)
- **F02-B1**: Verify clips are rendered correctly in horizontal track layout with duration badges.
- **F02-B2**: Verify clip selection highlights the active clip container.

### Zone C: Media Import (File System Access API)
- **F02-C1**: Select a local `.mp4` file via file picker. Verify that a new clip is appended to the virtual timeline array instantly with zero backend media upload.

### Zone D: Editing Toolbar (Trim, Split, Ripple Delete)
- **F02-D1 (Split)**: Select a clip and click "Split Clip". Verify the clip is divided into two adjacent virtual clips at the split point.
- **F02-D2 (Ripple Delete)**: Select a clip and click "Ripple Delete". Verify the clip is removed from the array and subsequent timeline clips shift leftward automatically.

### Zone E: Export Engine (FFmpeg.wasm)
- **F02-E1**: Click "Export MP4 (FFmpeg.wasm)". Verify client-side Web Worker processing starts and triggers final `.mp4` download.

---

## Flow 3: Cloud Sync (Vercel Serverless API)
- **F03-H1**: Click "Sync to Cloud (JSON)". Verify `POST /api/save_project` successfully transmits timeline JSON state to Vercel API and Supabase database.
