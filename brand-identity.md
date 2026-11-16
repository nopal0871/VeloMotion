# Brand Identity Report: VeloMotion (Formerly VeloMotion Clone)

## 1. Brand Overview & New Identity
- **App Name**: **VeloMotion**
- **Tagline**: Pro Client-Side WASM NLE & Cloud Sync Studio
- **Positioning**: A high-performance, browser-native video editing suite leveraging Leptos WASM, local Web Workers, FFmpeg.wasm, and Vercel Rust serverless APIs.

---

## 2. UI Color Palette
- **Primary Background**: Zinc 950 (`#09090b`) & Zinc 900 (`#18181b`)
- **Accent / Brand Color**: Neon Emerald Green (`#22c55e` / `green-500`) with subtle dark green glows (`green-950/60`).
- **Text & UI Elements**: Zinc 100 (`#f4f4f5`) for headers, Zinc 400 (`#a1a1aa`) for secondary labels, and red accent (`red-950`) for destructive operations (Ripple Delete).
- **Gradients**: Removed ambiguous multi-color gradients in favor of clean, professional dark-mode solid tones and minimalist borders.

---

## 3. Logo Brief
- **Concept**: Minimalist vector wordmark combined with a dynamic playhead/timeline vector node.
- **Mark Type**: Combination Mark (Monogram 'VM' + Playhead/Timeline nodes).
- **Scalability**: Designed to render sharply at 16x16px (favicon) and up to 1024x1024px (App Store / PWA icon).
- **Color Rules**: High-contrast emerald green on dark zinc background. Zero resemblance to legacy branding.

---

## 4. Brand Voice & Tone
- **Voice Pillars**: Precise, Responsive, Professional.
- **Do's**: Use direct operational terms ("Sync to Cloud", "Split Clip", "Ripple Delete", "Export MP4").
- **Don'ts**: Avoid cluttered jargon or legacy app terminology.

---

## 5. Codebase Rebranding & Sweep
- **Frontend Sweep**: All occurrences of legacy references in `frontend/src/views/dashboard.rs`, `frontend/src/views/editor.rs`, and `frontend/index.html` have been thoroughly updated to **VeloMotion**.
- **Backend Sweep**: Crate names and API routing structures verified clean.
- **Status**: **Clean Codebase** (No legacy trademark strings remaining in active application code).
