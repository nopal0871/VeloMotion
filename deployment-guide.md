# Deployment Guide & Preflight: VeloMotion

## 1. Preflight Verification
- **Codebase Sweep**: Clean (0 legacy brand references remaining).
- **Architecture Compliance**: Rust Fullstack (Leptos WASM client-side rendering + Vercel Rust Serverless backend + Supabase PostgreSQL).
- **Test Suite**: Playwright E2E and unit test specifications validated.

---

## 2. Frontend Deployment (GitHub Pages - Static Build)
VeloMotion's frontend is built entirely in Leptos (Rust WASM) and configured for static export.

### Prerequisites:
- Install Rust and Trunk:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup target add wasm32-unknown-unknown
  cargo install trunk
  ```

### Build Steps:
1. Navigate to the frontend directory:
   ```bash
   cd replica/frontend
   ```
2. Build the optimized release bundle:
   ```bash
   trunk build --release
   ```
   *This outputs compiled WASM and static assets into the `dist/` directory.*
3. Deploy `dist/` to GitHub Pages via GitHub Actions workflow or manual static publishing.

---

## 3. Backend Deployment (Vercel Serverless Functions - Rust)
The backend functions (`api/save_project`, `api/load_project`) handle JSON timeline persistence securely.

### Prerequisites:
- Install Vercel CLI:
  ```bash
  npm i -g vercel
  ```
- Configure Supabase PostgreSQL database and execute `replica/schema.sql`.

### Deployment Steps:
1. Navigate to the backend directory:
   ```bash
   cd replica/backend
   ```
2. Link your Vercel project:
   ```bash
   vercel link
   ```
3. Set the production environment variable for Supabase connection:
   ```bash
   vercel env add DATABASE_URL production
   # Enter your Supabase Postgres connection string when prompted
   ```
4. Deploy to production:
   ```bash
   vercel --prod
   ```

---

## 4. Post-Deployment Verification
- Verify GitHub Pages URL loads the VeloMotion Dashboard and Editor successfully.
- Test creating a project, editing clips on the timeline (Split / Ripple Delete), and syncing JSON state to the Vercel Rust backend API.
