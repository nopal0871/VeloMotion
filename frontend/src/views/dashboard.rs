use leptos::*;
use leptos_router::*;
<<<<<<< HEAD
use crate::models::*;
use crate::storage::*;
use crate::effects::*;
use chrono::Utc;
use uuid::Uuid;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let (projects, set_projects) = create_signal(load_projects());
    let (custom_effects, set_custom_effects) = create_signal(load_custom_effects());
    
    // Modals state
    let (show_create_modal, set_show_create_modal) = create_signal(false);
    let (show_effect_modal, set_show_effect_modal) = create_signal(false);

    // New project form state
    let (new_title, set_new_title) = create_signal("My Awesome Video".to_string());
    let (aspect_choice, set_aspect_choice) = create_signal("16:9".to_string());
    let (custom_w, set_custom_w) = create_signal(1080u32);
    let (custom_h, set_custom_h) = create_signal(1920u32);
    let (resolution_choice, set_resolution_choice) = create_signal(Resolution::P1080);
    let (fps_choice, set_fps_choice) = create_signal(30u32);
    let (bg_choice, set_bg_choice) = create_signal(CanvasBackground::Black);

    // Custom effect form state
    let (effect_name, set_effect_name) = create_signal("New Custom Effect".to_string());
    let (effect_code, set_effect_code) = create_signal("// Write your custom filter logic\nctx.set_filter(\"sepia(100%)\");".to_string());

    let navigate = store_value(use_navigate());

    // Handlers
    let create_project_handler = move |_| {
        let ar = match aspect_choice.get().as_str() {
            "16:9" => AspectRatioPreset::Landscape16_9,
            "9:16" => AspectRatioPreset::Portrait9_16,
            "1:1" => AspectRatioPreset::Square1_1,
            "4:5" => AspectRatioPreset::Feed4_5,
            _ => AspectRatioPreset::Custom(custom_w.get(), custom_h.get()),
        };

        let new_proj = Project {
            id: Uuid::new_v4().to_string(),
            title: if new_title.get().is_empty() { "Untitled Project".to_string() } else { new_title.get() },
            aspect_ratio: ar,
            resolution: resolution_choice.get(),
            fps: fps_choice.get(),
            bg_color: bg_choice.get(),
            created_at: Utc::now().to_rfc3339(),
            last_opened: Utc::now().to_rfc3339(),
            duration_used_seconds: 0,
            tracks: vec![
                Track {
                    id: Uuid::new_v4().to_string(),
                    name: "Main Video Track".to_string(),
                    track_type: TrackType::Video,
                    clips: vec![],
                },
                Track {
                    id: Uuid::new_v4().to_string(),
                    name: "Audio Track".to_string(),
                    track_type: TrackType::Audio,
                    clips: vec![],
                },
            ],
            beat_markers: vec![],
        };

        let mut current = projects.get();
        current.push(new_proj.clone());
        save_projects(&current);
        set_projects.set(current);
        set_show_create_modal.set(false);

        // Navigate to editor
        navigate.with_value(|nav| {
            let _ = nav(&format!("/editor/{}", new_proj.id), Default::default());
        });
    };

    let duplicate_project = move |id: String| {
        let mut current = projects.get();
        if let Some(proj) = current.iter().find(|p| p.id == id).cloned() {
            let mut dup = proj;
            dup.id = Uuid::new_v4().to_string();
            dup.title = format!("{} (Copy)", dup.title);
            dup.created_at = Utc::now().to_rfc3339();
            current.push(dup);
            save_projects(&current);
            set_projects.set(current);
        }
    };

    let delete_project = move |id: String| {
        let mut current = projects.get();
        current.retain(|p| p.id != id);
        save_projects(&current);
        set_projects.set(current);
    };

    let add_custom_effect_handler = move |_| {
        let plugin = CustomEffectPlugin {
            id: format!("custom_{}", Uuid::new_v4()),
            name: effect_name.get(),
            code: effect_code.get(),
        };
        let mut current = custom_effects.get();
        current.push(plugin);
        save_custom_effects(&current);
        set_custom_effects.set(current);
        set_show_effect_modal.set(false);
    };

    let built_ins = get_built_in_effects();
    let built_in_count = built_ins.len();

    view! {
        <div class="min-h-screen bg-gray-950 text-gray-100 flex flex-col">
            // Top Header with Modular Effect Hub (Top-Right)
            <header class="bg-gray-900 border-b border-gray-800 px-6 py-4 flex flex-wrap justify-between items-center shadow-lg">
                <div class="flex items-center space-x-3">
                    <div class="bg-indigo-600 text-white font-bold p-2.5 rounded-xl shadow-md flex items-center justify-center text-xl">
                        "🎬"
                    </div>
                    <div>
                        <h1 class="text-2xl font-black tracking-tight text-white">"Rust Web Video Editor"</h1>
                        <p class="text-xs text-gray-400">"Client-side 2-Page Web Video Editor Engine in Rust (WASM + Leptos)"</p>
                    </div>
                </div>

                // Modular Effect Manager Top-Right Section
                <div class="flex items-center space-x-4 mt-3 sm:mt-0">
                    <button
                        on:click=move |_| set_show_effect_modal.set(true)
                        class="bg-gray-800 hover:bg-gray-700 text-indigo-400 border border-indigo-500/30 px-4 py-2 rounded-lg font-medium text-sm transition flex items-center space-x-2 shadow"
                    >
                        <span>"⚡ Modular Effect Hub"</span>
                        <span class="bg-indigo-950 text-indigo-300 text-xs px-2 py-0.5 rounded-full">
                            {move || built_in_count + custom_effects.get().len()}
                        </span>
                    </button>

                    <button
                        on:click=move |_| set_show_create_modal.set(true)
                        class="bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white font-bold px-5 py-2.5 rounded-lg shadow-lg flex items-center space-x-2 transition transform active:scale-95"
                    >
                        <span>"+ New Project"</span>
                    </button>
                </div>
            </header>

            // Main Content: Project History Dashboard
            <main class="flex-1 max-w-7xl w-full mx-auto p-6 space-y-8">
                <div class="flex justify-between items-center border-b border-gray-800 pb-4">
                    <div>
                        <h2 class="text-xl font-bold text-white">"Project Dashboard & History"</h2>
                        <p class="text-sm text-gray-400">"Select an existing project to edit or create a new video workspace."</p>
                    </div>
                </div>

                // Projects Grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {move || {
                        projects.get().into_iter().map(|proj| {
                            let proj_id_open = proj.id.clone();
                            let proj_id_dup = proj.id.clone();
                            let proj_id_del = proj.id.clone();
                            let dur_secs = proj.duration_used_seconds;
                            let hours = dur_secs / 3600;
                            let mins = (dur_secs % 3600) / 60;
                            let secs = dur_secs % 60;
                            let duration_str = format!("{:02}:{:02}:{:02}", hours, mins, secs);

                            view! {
                                <div class="bg-gray-900 border border-gray-800 rounded-2xl p-5 hover:border-indigo-500/50 transition shadow-xl flex flex-col justify-between group">
                                    <div>
                                        <div class="flex justify-between items-start mb-3">
                                            <span class="bg-indigo-900/60 text-indigo-300 border border-indigo-700/50 text-xs font-semibold px-2.5 py-1 rounded-md">
                                                {proj.aspect_ratio.to_string()}
                                            </span>
                                            <span class="text-xs text-gray-500 bg-gray-800 px-2 py-0.5 rounded">
                                                {proj.resolution.label()} " @ " {proj.fps} "fps"
                                            </span>
                                        </div>
                                        <h3 class="text-lg font-bold text-white group-hover:text-indigo-400 transition truncate">
                                            {proj.title}
                                        </h3>
                                        <div class="mt-4 space-y-1.5 text-xs text-gray-400">
                                            <div class="flex justify-between">
                                                <span>"Created:"</span>
                                                <span class="text-gray-300">{proj.created_at.chars().take(10).collect::<String>()}</span>
                                            </div>
                                            <div class="flex justify-between">
                                                <span>"Last Opened:"</span>
                                                <span class="text-gray-300">{proj.last_opened.chars().take(10).collect::<String>()}</span>
                                            </div>
                                            <div class="flex justify-between">
                                                <span>"Editing Time Used:"</span>
                                                <span class="text-indigo-400 font-mono font-bold">{duration_str}</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="mt-6 pt-4 border-t border-gray-800 flex items-center justify-between gap-2">
                                        <a
                                            href=format!("{}/editor/{}", crate::config::BASE_PATH, proj_id_open)
                                            class="flex-1 bg-indigo-600 hover:bg-indigo-500 text-white text-center font-medium py-2 px-3 rounded-lg text-sm transition shadow"
                                        >
                                            "Open Project"
                                        </a>
                                        <button
                                            on:click=move |_| duplicate_project(proj_id_dup.clone())
                                            class="bg-gray-800 hover:bg-gray-700 text-gray-300 p-2 rounded-lg text-sm transition"
                                            title="Duplicate Project"
                                        >
                                            "📋"
                                        </button>
                                        <button
                                            on:click=move |_| delete_project(proj_id_del.clone())
                                            class="bg-red-950/40 hover:bg-red-900/60 text-red-400 p-2 rounded-lg text-sm transition border border-red-900/30"
                                            title="Delete Project"
                                        >
                                            "🗑️"
                                        </button>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </main>

            // PROJECT CREATION MODAL
            {move || show_create_modal.get().then(|| view! {
                <div class="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50 p-4">
                    <div class="bg-gray-900 border border-gray-800 w-full max-w-lg rounded-2xl p-6 shadow-2xl space-y-6">
                        <div class="flex justify-between items-center border-b border-gray-800 pb-4">
                            <h3 class="text-xl font-bold text-white">"Create New Video Project"</h3>
                            <button
                                on:click=move |_| set_show_create_modal.set(false)
                                class="text-gray-400 hover:text-white text-xl font-bold"
                            >
                                "✕"
                            </button>
                        </div>

                        <div class="space-y-4">
                            <div>
                                <label class="block text-xs font-semibold text-gray-300 uppercase tracking-wider mb-1">"Project Title"</label>
                                <input
                                    type="text"
                                    prop:value=move || new_title.get()
                                    on:input=move |ev| set_new_title.set(event_target_value(&ev))
                                    class="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-indigo-500"
                                />
                            </div>

                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-xs font-semibold text-gray-300 uppercase tracking-wider mb-1">"Aspect Ratio Preset"</label>
                                    <select
                                        on:change=move |ev| set_aspect_choice.set(event_target_value(&ev))
                                        class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-2.5 text-white focus:outline-none focus:border-indigo-500"
                                    >
                                        <option value="16:9">"16:9 Landscape"</option>
                                        <option value="9:16">"9:16 Portrait"</option>
                                        <option value="1:1">"1:1 Square"</option>
                                        <option value="4:5">"4:5 Feed"</option>
                                        <option value="Custom">"Custom Aspect Ratio"</option>
                                    </select>
                                </div>

                                <div>
                                    <label class="block text-xs font-semibold text-gray-300 uppercase tracking-wider mb-1">"Resolution"</label>
                                    <select
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            let res = match val.as_str() {
                                                "720p" => Resolution::P720,
                                                "1440p" => Resolution::P1440,
                                                "4K" => Resolution::K4,
                                                _ => Resolution::P1080,
                                            };
                                            set_resolution_choice.set(res);
                                        }
                                        class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-2.5 text-white focus:outline-none focus:border-indigo-500"
                                    >
                                        <option value="1080p">"1080p (Full HD)"</option>
                                        <option value="720p">"720p (HD)"</option>
                                        <option value="1440p">"1440p (2K)"</option>
                                        <option value="4K">"4K (Ultra HD)"</option>
                                    </select>
                                </div>
                            </div>

                            {move || (aspect_choice.get() == "Custom").then(|| view! {
                                <div class="grid grid-cols-2 gap-4 bg-gray-950 p-3 rounded-lg border border-gray-800">
                                    <div>
                                        <label class="block text-xs font-semibold text-gray-400 mb-1">"Custom Width"</label>
                                        <input
                                            type="number"
                                            prop:value=move || custom_w.get()
                                            on:input=move |ev| set_custom_w.set(event_target_value(&ev).parse().unwrap_or(1080))
                                            class="w-full bg-gray-900 border border-gray-800 rounded px-3 py-1.5 text-white"
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-semibold text-gray-400 mb-1">"Custom Height"</label>
                                        <input
                                            type="number"
                                            prop:value=move || custom_h.get()
                                            on:input=move |ev| set_custom_h.set(event_target_value(&ev).parse().unwrap_or(1920))
                                            class="w-full bg-gray-900 border border-gray-800 rounded px-3 py-1.5 text-white"
                                        />
                                    </div>
                                </div>
                            })}

                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-xs font-semibold text-gray-300 uppercase tracking-wider mb-1">"Framerate (FPS)"</label>
                                    <select
                                        on:change=move |ev| set_fps_choice.set(event_target_value(&ev).parse().unwrap_or(30))
                                        class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-2.5 text-white focus:outline-none focus:border-indigo-500"
                                    >
                                        <option value="30">"30 FPS"</option>
                                        <option value="24">"24 FPS (Cinematic)"</option>
                                        <option value="60">"60 FPS (Smooth)"</option>
                                    </select>
                                </div>

                                <div>
                                    <label class="block text-xs font-semibold text-gray-300 uppercase tracking-wider mb-1">"Canvas Background"</label>
                                    <select
                                        on:change=move |ev| {
                                            let bg = match event_target_value(&ev).as_str() {
                                                "White" => CanvasBackground::White,
                                                "Green" => CanvasBackground::Green,
                                                "Blue" => CanvasBackground::Blue,
                                                _ => CanvasBackground::Black,
                                            };
                                            set_bg_choice.set(bg);
                                        }
                                        class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-2.5 text-white focus:outline-none focus:border-indigo-500"
                                    >
                                        <option value="Black">"Black (#000000)"</option>
                                        <option value="White">"White (#FFFFFF)"</option>
                                        <option value="Green">"Green (#00FF00)"</option>
                                        <option value="Blue">"Blue (#0000FF)"</option>
                                    </select>
                                </div>
                            </div>
                        </div>

                        <div class="flex justify-end space-x-3 pt-4 border-t border-gray-800">
                            <button
                                on:click=move |_| set_show_create_modal.set(false)
                                class="px-4 py-2 bg-gray-800 hover:bg-gray-700 text-gray-300 rounded-lg font-medium transition"
                            >
                                "Cancel"
                            </button>
                            <button
                                on:click=create_project_handler
                                class="px-5 py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded-lg font-bold shadow-lg transition"
                            >
                                "Create Project"
                            </button>
                        </div>
                    </div>
                </div>
            })}

            // MODULAR EFFECT MANAGER & CODE SNIPPET EDITOR MODAL
            {move || show_effect_modal.get().then(|| view! {
                <div class="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50 p-4">
                    <div class="bg-gray-900 border border-gray-800 w-full max-w-2xl rounded-2xl p-6 shadow-2xl space-y-6">
                        <div class="flex justify-between items-center border-b border-gray-800 pb-4">
                            <div>
                                <h3 class="text-xl font-bold text-white">"Modular Effect Manager & Hub"</h3>
                                <p class="text-xs text-gray-400">"View registered video effects and load custom Rust/WASM effect plugins."</p>
                            </div>
                            <button
                                on:click=move |_| set_show_effect_modal.set(false)
                                class="text-gray-400 hover:text-white text-xl font-bold"
                            >
                                "✕"
                            </button>
                        </div>

                        <div class="space-y-4 max-h-[60vh] overflow-y-auto pr-1">
                            <div>
                                <h4 class="text-sm font-semibold text-indigo-400 mb-2">"Built-in Effect Registry"</h4>
                                <div class="grid grid-cols-2 gap-3">
                                    {built_ins.iter().map(|eff| view! {
                                        <div class="bg-gray-950 border border-gray-800 p-3 rounded-xl flex flex-col justify-between">
                                            <div>
                                                <div class="font-bold text-white text-sm">{eff.name()}</div>
                                                <div class="text-xs text-gray-500 font-mono mt-0.5">{eff.id()}</div>
                                            </div>
                                            <span class="mt-3 text-xs bg-emerald-950 text-emerald-400 border border-emerald-900/50 px-2 py-0.5 rounded text-center font-medium">
                                                "Active Module"
                                            </span>
                                        </div>
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>

                            <div class="border-t border-gray-800 pt-4">
                                <h4 class="text-sm font-semibold text-indigo-400 mb-2">"Custom Rust / WASM Effect Plugins"</h4>
                                <div class="space-y-3">
                                    {custom_effects.get().into_iter().map(|plug| view! {
                                        <div class="bg-gray-950 border border-gray-800 p-3 rounded-xl">
                                            <div class="flex justify-between items-center mb-1">
                                                <span class="font-bold text-white text-sm">{plug.name}</span>
                                                <span class="text-xs text-purple-400 font-mono">{plug.id}</span>
                                            </div>
                                            <pre class="bg-gray-900 p-2.5 rounded-lg text-xs font-mono text-gray-300 overflow-x-auto">{plug.code}</pre>
                                        </div>
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>

                            <div class="border-t border-gray-800 pt-4 space-y-3">
                                <h4 class="text-sm font-semibold text-purple-400 mb-2">"Register New Custom Effect Snippet"</h4>
                                <div>
                                    <label class="block text-xs font-semibold text-gray-300 mb-1">"Effect Name"</label>
                                    <input
                                        type="text"
                                        prop:value=move || effect_name.get()
                                        on:input=move |ev| set_effect_name.set(event_target_value(&ev))
                                        class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-2 text-white text-sm"
                                    />
                                </div>
                                <div>
                                    <label class="block text-xs font-semibold text-gray-300 mb-1">"Rust/WASM Effect Code Snippet"</label>
                                    <textarea
                                        prop:value=move || effect_code.get()
                                        on:input=move |ev| set_effect_code.set(event_target_value(&ev))
                                        rows="3"
                                        class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-2 text-white text-xs font-mono"
                                    ></textarea>
                                </div>
                                <button
                                    on:click=add_custom_effect_handler
                                    class="w-full bg-purple-600 hover:bg-purple-500 text-white font-bold py-2.5 rounded-lg text-sm transition shadow"
                                >
                                    "Register Custom Effect Plugin"
                                </button>
                            </div>
                        </div>

                        <div class="flex justify-end pt-3 border-t border-gray-800">
                            <button
                                on:click=move |_| set_show_effect_modal.set(false)
                                class="px-5 py-2 bg-gray-800 hover:bg-gray-700 text-gray-300 rounded-lg font-medium transition"
                            >
                                "Close Hub"
                            </button>
                        </div>
                    </div>
                </div>
            })}
=======
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProjectSummary {
    pub id: String,
    pub title: String,
    pub updated_at: String,
    pub duration_secs: f64,
}

#[component]
pub fn DashboardView() -> impl IntoView {
    // State for projects list (seed data for zero-backend fallback, fetched from Supabase via Vercel API in production)
    let (projects, _set_projects) = create_signal(vec![
        ProjectSummary {
            id: "proj-1".to_string(),
            title: "Neon Cinematic Intro 1080p".to_string(),
            updated_at: "2026-11-13 14:30".to_string(),
            duration_secs: 15.4,
        },
        ProjectSummary {
            id: "proj-2".to_string(),
            title: "TikTok Kinetic Typography Reel".to_string(),
            updated_at: "2026-11-12 09:15".to_string(),
            duration_secs: 28.0,
        },
    ]);

    let navigate = use_navigate();

    let create_new_project = move |_| {
        let new_id = uuid::Uuid::new_v4().to_string();
        let path = format!("/editor/{}", new_id);
        navigate(&path, Default::default());
    };

    view! {
        <div class="max-w-7xl mx-auto px-6 py-10">
            // Header
            <header class="flex flex-col md:flex-row justify-between items-start md:items-center pb-8 border-b border-zinc-800 gap-4">
                <div>
                    <h1 class="text-3xl font-extrabold tracking-tight text-white flex items-center gap-3">
                        <span class="inline-block w-3 h-3 rounded-full bg-green-500 animate-pulse"></span>
                        "VeloMotion Studio"
                    </h1>
                    <p class="text-sm text-zinc-400 mt-1">
                        "Client-Side WASM NLE & Vercel Rust Cloud Sync"
                    </p>
                </div>
                <button
                    on:click=create_new_project
                    class="bg-green-600 hover:bg-green-500 text-white font-semibold px-6 py-3 rounded-xl shadow-lg shadow-green-900/30 transition-all flex items-center gap-2 transform active:scale-95"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
                    </svg>
                    "Create New Video Project"
                </button>
            </header>

            // Main Content: Recent Projects Grid
            <section class="mt-10">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-xl font-bold text-zinc-200">"Recent Video Projects"</h2>
                    <span class="text-xs text-zinc-500 uppercase tracking-wider font-semibold">"Synced with Supabase"</span>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <For
                        each=projects
                        key=|p| p.id.clone()
                        children=move |p| {
                            let pid = p.id.clone();
                            let edit_url = format!("/editor/{}", pid);
                            view! {
                                <div class="bg-zinc-900/80 border border-zinc-800 rounded-2xl p-5 hover:border-green-500/50 transition-all group flex flex-col justify-between">
                                    <div>
                                        <div class="aspect-video bg-zinc-950 rounded-xl mb-4 border border-zinc-800 flex items-center justify-center relative overflow-hidden group-hover:border-zinc-700">
                                            <div class="absolute inset-0 bg-gradient-to-tr from-green-900/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 text-zinc-700 group-hover:text-green-500 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                            <span class="absolute bottom-2 right-2 bg-zinc-900/90 text-xs px-2 py-1 rounded text-zinc-300 font-mono">
                                                {format!("{:.1}s", p.duration_secs)}
                                            </span>
                                        </div>
                                        <h3 class="font-bold text-lg text-zinc-100 group-hover:text-green-400 transition-colors truncate">
                                            {p.title}
                                        </h3>
                                        <p class="text-xs text-zinc-500 mt-1">
                                            "Updated " {p.updated_at}
                                        </p>
                                    </div>

                                    <div class="mt-6 pt-4 border-t border-zinc-800/80 flex justify-between items-center">
                                        <span class="text-xs font-mono text-zinc-600">{"ID: "}{p.id}</span>
                                        <A
                                            href=edit_url
                                            class="bg-zinc-800 hover:bg-green-600 text-zinc-200 hover:text-white px-4 py-2 rounded-lg text-sm font-semibold transition-all flex items-center gap-1.5"
                                        >
                                            "Open Editor"
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                                                <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                                            </svg>
                                        </A>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </section>
>>>>>>> 3c3702960cb7d26ba2cdfe0d5e737ed02cd3374f
        </div>
    }
}
