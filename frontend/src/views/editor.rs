use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct VideoClip {
    pub id: String,
    pub name: String,
    pub duration: f64,     // in seconds
    pub trim_in: f64,      // start offset
    pub trim_out: f64,     // end offset
    pub object_url: String, // local blob url
}

#[component]
pub fn EditorView() -> impl IntoView {
    let params = use_params_map();
    let project_id = move || params.with(|p| p.get("id").cloned().unwrap_or_else(|| "new-project".to_string()));

    // Virtual state for timeline clips using Leptos create_rw_signal
    let clips = create_rw_signal(vec![
        VideoClip {
            id: "clip-1".to_string(),
            name: "sample_footage_01.mp4".to_string(),
            duration: 10.0,
            trim_in: 0.0,
            trim_out: 10.0,
            object_url: "#".to_string(),
        },
        VideoClip {
            id: "clip-2".to_string(),
            name: "cinematic_transition_fx.mp4".to_string(),
            duration: 5.5,
            trim_in: 0.0,
            trim_out: 5.5,
            object_url: "#".to_string(),
        },
    ]);

    let selected_clip_id = create_rw_signal(None::<String>);
    let is_playing = create_rw_signal(false);
    let playhead_time = create_rw_signal(0.0);

    // Zone C: Handle local file import (Zero backend upload)
    let on_file_selected = move |ev: web_sys::Event| {
        let input = event_target::<web_sys::HtmlInputElement>(ev);
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let name = file.name();
                let url = web_sys::Url::create_object_url_with_blob(&file).unwrap_or_default();
                let new_clip = VideoClip {
                    id: uuid::Uuid::new_v4().to_string(),
                    name,
                    duration: 8.0, // Default estimate until loaded
                    trim_in: 0.0,
                    trim_out: 8.0,
                    object_url: url,
                };
                clips.update(|c| c.push(new_clip));
            }
        }
    };

    // Zone D: Editing Operations
    let split_clip = move |_| {
        if let Some(id) = selected_clip_id.get() {
            clips.update(|list| {
                if let Some(pos) = list.iter().position(|c| c.id == id) {
                    let original = &list[pos];
                    let split_point = (original.trim_out - original.trim_in) / 2.0;
                    let mut second_half = original.clone();
                    second_half.id = uuid::Uuid::new_v4().to_string();
                    second_half.trim_in = original.trim_in + split_point;
                    
                    list[pos].trim_out = original.trim_in + split_point;
                    list.insert(pos + 1, second_half);
                }
            });
        }
    };

    let delete_clip = move |_| {
        if let Some(id) = selected_clip_id.get() {
            clips.update(|list| {
                list.retain(|c| c.id != id);
            });
            selected_clip_id.set(None);
        }
    };

    let save_project = move |_| {
        // Triggers Vercel Rust API json save
        let _current_clips = clips.get();
        gloo_timers::future::TimeoutFuture::new(500).await;
        // In real app, post to /api/save_project
    };

    let navigate = use_navigate();

    view! {
        <div class="flex flex-col h-screen bg-zinc-950 text-zinc-100 overflow-hidden">
            // Top Navigation Bar
            <header class="h-14 border-b border-zinc-800 bg-zinc-900/90 px-6 flex items-center justify-between shrink-0">
                <div class="flex items-center gap-4">
                    <button
                        on:click=move |_| navigate("/", Default::default())
                        class="text-zinc-400 hover:text-white transition-colors flex items-center gap-1.5 text-sm font-medium"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z" clip-rule="evenodd" />
                        </svg>
                        "Dashboard"
                    </button>
                    <span class="text-zinc-700">"/"</span>
                    <span class="text-sm font-semibold text-zinc-200">
                        {"Project ID: "}{project_id()}
                    </span>
                </div>

                <div class="flex items-center gap-3">
                    <button
                        on:click=save_project
                        class="bg-zinc-800 hover:bg-zinc-700 text-zinc-200 text-xs font-semibold px-3 py-1.5 rounded-lg border border-zinc-700 transition-all flex items-center gap-1.5"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5 text-green-400" viewBox="0 0 20 20" fill="currentColor">
                            <path d="M7.707 10.293a1 1 0 10-1.414 1.414l3 3a1 1 0 001.414 0l3-3a1 1 0 00-1.414-1.414L11 11.586V6h-2v5.586l-1.293-1.293z" />
                            <path d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" />
                        </svg>
                        "Sync to Cloud (JSON)"
                    </button>
                </div>
            </header>

            // Main Editor Workspace Grid (Zones A, B, C, D, E)
            <div class="flex-1 grid grid-cols-1 lg:grid-cols-4 overflow-hidden">
                
                // Left / Center Main Area (Zones A & B)
                <div class="lg:col-span-3 flex flex-col h-full border-r border-zinc-800 overflow-hidden">
                    
                    // Zone A: Video Canvas Preview
                    <div class="flex-1 bg-zinc-950 flex flex-col items-center justify-center p-6 relative min-h-[300px]">
                        <div class="absolute top-4 left-4 flex items-center gap-2">
                            <span class="text-xs font-mono uppercase bg-green-500/10 text-green-400 border border-green-500/30 px-2 py-0.5 rounded">
                                "Zone A: Canvas Preview (requestAnimationFrame)"
                            </span>
                        </div>

                        // HTML5 Canvas for high performance rendering
                        <div class="aspect-video w-full max-w-3xl bg-zinc-900 rounded-xl border border-zinc-800 shadow-2xl flex items-center justify-center relative overflow-hidden group">
                            <canvas id="preview-canvas" class="w-full h-full object-contain"></canvas>
                            <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
                                <div class="text-center">
                                    <p class="text-sm font-medium text-zinc-400">"VeloMotion Compositing Engine"</p>
                                    <p class="text-xs text-zinc-600 font-mono mt-1">"Time: {move || format!("{:.2}s", playhead_time.get())}"</p>
                                </div>
                            </div>
                        </div>

                        // Playback Control bar
                        <div class="mt-4 flex items-center gap-4 bg-zinc-900/80 border border-zinc-800 px-4 py-2 rounded-xl">
                            <button
                                on:click=move |_| is_playing.update(|p| *p = !*p)
                                class="w-10 h-10 rounded-lg bg-green-600 hover:bg-green-500 text-white flex items-center justify-center transition-all shadow-md shadow-green-900/20"
                            >
                                {move || if is_playing.get() {
                                    view! {
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
                                        </svg>
                                    }
                                } else {
                                    view! {
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 translate-x-0.5" viewBox="0 0 20 20" fill="currentColor">
                                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
                                        </svg>
                                    }
                                }}
                            </button>
                            <span class="text-xs font-mono text-zinc-400">
                                {move || if is_playing.get() { "Playing (60 FPS rAF)" } else { "Paused" }}
                            </span>
                        </div>
                    </div>

                    // Zone B: Video Timeline NLE
                    <div class="h-64 bg-zinc-900/90 border-t border-zinc-800 flex flex-col p-4 shrink-0">
                        <div class="flex justify-between items-center mb-3">
                            <span class="text-xs font-mono uppercase bg-zinc-800 text-zinc-300 px-2 py-0.5 rounded border border-zinc-700">
                                "Zone B: Video Timeline NLE (Virtual Vec<VideoClip>)"
                            </span>
                            <span class="text-xs text-zinc-500 font-mono">
                                {move || format!("Total Clips: {}", clips.with(|c| c.len()))}
                            </span>
                        </div>

                        // Timeline Tracks Container
                        <div class="flex-1 bg-zinc-950 rounded-xl border border-zinc-800 p-3 overflow-x-auto relative flex flex-col gap-2">
                            <For
                                each=move || clips.get()
                                key=|c| c.id.clone()
                                children=move |clip| {
                                    let cid = clip.id.clone();
                                    let is_selected = move || selected_clip_id.get() == Some(cid.clone());
                                    let select_cb = move |_| {
                                        selected_clip_id.set(Some(cid.clone()));
                                    };
                                    view! {
                                        <div
                                            on:click=select_cb
                                            class=move || format!(
                                                "h-12 rounded-lg px-4 flex items-center justify-between cursor-pointer border transition-all {}",
                                                if is_selected() {
                                                    "bg-green-950/60 border-green-500 shadow-md shadow-green-900/20 text-green-200"
                                                } else {
                                                    "bg-zinc-900 border-zinc-800 hover:border-zinc-700 text-zinc-300"
                                                }
                                            )
                                        >
                                            <div class="flex items-center gap-3 truncate">
                                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-green-500 shrink-0" viewBox="0 0 20 20" fill="currentColor">
                                                    <path fill-rule="evenodd" d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm3 2h6v4H7V5zm8 8v2h-2v-2h2zm-4 2H7v-2h2v2zm0-4H7V7h2v2zm4 0h-2V7h2v2z" clip-rule="evenodd" />
                                                </svg>
                                                <span class="text-sm font-medium truncate">{clip.name}</span>
                                            </div>
                                            <span class="text-xs font-mono text-zinc-500">
                                                {format!("{:.1}s", clip.duration)}
                                            </span>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                </div>

                // Right Sidebar (Zones C, D, E)
                <div class="flex flex-col h-full bg-zinc-900/60 p-5 gap-6 overflow-y-auto">
                    
                    // Zone C: Media Import
                    <div class="bg-zinc-900 border border-zinc-800 rounded-2xl p-4">
                        <div class="flex items-center justify-between mb-3">
                            <span class="text-xs font-mono uppercase text-green-400 font-semibold">
                                "Zone C: Media Import"
                            </span>
                        </div>
                        <p class="text-xs text-zinc-400 mb-3">
                            "Load local video/audio files via File System Access API (Zero server upload)."
                        </p>
                        <label class="w-full bg-zinc-800 hover:bg-zinc-700 text-zinc-200 border border-zinc-700 rounded-xl py-2.5 px-4 text-sm font-semibold flex items-center justify-center gap-2 cursor-pointer transition-all">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-green-400" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM6.293 6.707a1 1 0 010-1.414l3-3a1 1 0 011.414 0l3 3a1 1 0 01-1.414 1.414L11 5.414V13a1 1 0 11-2 0V5.414L7.707 6.707a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                            </svg>
                            "Import Local Media"
                            <input type="file" accept="video/*,audio/*" on:change=on_file_selected class="hidden" />
                        </label>
                    </div>

                    // Zone D: Editing Toolbar
                    <div class="bg-zinc-900 border border-zinc-800 rounded-2xl p-4">
                        <div class="flex items-center justify-between mb-3">
                            <span class="text-xs font-mono uppercase text-green-400 font-semibold">
                                "Zone D: Editing Toolbar"
                            </span>
                        </div>
                        <div class="grid grid-cols-2 gap-2">
                            <button
                                on:click=split_clip
                                class="bg-zinc-800 hover:bg-zinc-700 text-zinc-200 border border-zinc-700 py-2 px-3 rounded-xl text-xs font-semibold flex items-center justify-center gap-1.5 transition-all"
                            >
                                "Split Clip"
                            </button>
                            <button
                                on:click=delete_clip
                                class="bg-red-950/40 hover:bg-red-900/60 text-red-300 border border-red-900/50 py-2 px-3 rounded-xl text-xs font-semibold flex items-center justify-center gap-1.5 transition-all"
                            >
                                "Ripple Delete"
                            </button>
                        </div>
                    </div>

                    // Zone E: Export Engine
                    <div class="bg-zinc-900 border border-zinc-800 rounded-2xl p-4 mt-auto">
                        <div class="flex items-center justify-between mb-3">
                            <span class="text-xs font-mono uppercase text-green-400 font-semibold">
                                "Zone E: Export Engine"
                            </span>
                        </div>
                        <p class="text-xs text-zinc-400 mb-4">
                            "Render final MP4 locally via FFmpeg.wasm stream-copy."
                        </p>
                        <button
                            class="w-full bg-green-600 hover:bg-green-500 text-white font-bold py-3 px-4 rounded-xl shadow-lg shadow-green-900/30 transition-all flex items-center justify-center gap-2"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                            "Export MP4 (FFmpeg.wasm)"
                        </button>
                    </div>

                </div>

            </div>
        </div>
    }
}
