use leptos::*;
use leptos_router::*;
use crate::models::*;
use crate::storage::*;
use crate::effects::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use std::collections::HashMap;
use uuid::Uuid;

// Simpan proyek yang sedang diedit tanpa menimpa proyek lain di storage
fn persist_project(current: &Project) {
    let mut all = load_projects();
    if let Some(slot) = all.iter_mut().find(|p| p.id == current.id) {
        *slot = current.clone();
    } else {
        all.push(current.clone());
    }
    save_projects(&all);
}

#[component]
pub fn EditorPage() -> impl IntoView {
    let params = use_params_map();
    let project_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let projects = load_projects();
    let current_project = projects.into_iter().find(|p| p.id == project_id()).unwrap_or_else(|| {
        Project {
            id: project_id(),
            title: "Untitled Video".to_string(),
            aspect_ratio: AspectRatioPreset::Landscape16_9,
            resolution: Resolution::P1080,
            fps: 30,
            bg_color: CanvasBackground::Black,
            created_at: "".to_string(),
            last_opened: "".to_string(),
            duration_used_seconds: 0,
            tracks: vec![
                Track {
                    id: Uuid::new_v4().to_string(),
                    name: "Video Track".to_string(),
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
        }
    });

    let (project, set_project) = create_signal(current_project);
    let (playhead_time, set_playhead_time) = create_signal(0.0f64); // in seconds
    let (is_playing, set_is_playing) = create_signal(false);
    let (selected_clip_id, set_selected_clip_id) = create_signal(Option::<String>::None);

    let canvas_ref = create_node_ref::<html::Canvas>();

    // Playback loop: majukan playhead saat tombol Play aktif
    if let Ok(handle) = set_interval_with_handle(
        move || {
            if is_playing.get_untracked() {
                set_playhead_time.update(|t| *t += 1.0 / 30.0);
            }
        },
        std::time::Duration::from_millis(33),
    ) {
        on_cleanup(move || handle.clear());
    }

    // Canvas render effect loop
    create_effect(move |_| {
        let proj = project.get();
        let time = playhead_time.get();
        
        if let Some(canvas) = canvas_ref.get() {
            let context = canvas
                .get_context("2d")
                .ok()
                .flatten()
                .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok());

            if let Some(ctx) = context {
                let width = canvas.width() as f64;
                let height = canvas.height() as f64;

                // Clear background
                ctx.set_fill_style(&JsValue::from_str(proj.bg_color.hex()));
                ctx.fill_rect(0.0, 0.0, width, height);
                ctx.set_filter("none");

                // Render tracks and clips active at current playhead time
                for track in &proj.tracks {
                    for clip in &track.clips {
                        if time >= clip.start_time && time <= (clip.start_time + clip.duration) {
                            ctx.save();
                            ctx.set_global_alpha(clip.opacity / 100.0);
                            
                            // Apply rotation around center
                            let cx = clip.x + clip.w / 2.0;
                            let cy = clip.y + clip.h / 2.0;
                            let _ = ctx.translate(cx, cy);
                            let _ = ctx.rotate(clip.rotation * std::f64::consts::PI / 180.0);
                            let _ = ctx.translate(-cx, -cy);

                            // Apply modular effects on canvas
                            for eff in &clip.effects {
                                if eff.enabled {
                                    apply_effect_by_id(&eff.effect_id, &ctx, width, height, time, &eff.params);
                                }
                            }

                            // Draw clip shape / type
                            match &clip.clip_type {
                                ClipType::Rectangle => {
                                    ctx.set_fill_style(&JsValue::from_str("#6366f1")); // Indigo
                                    ctx.fill_rect(clip.x, clip.y, clip.w, clip.h);
                                }
                                ClipType::Circle => {
                                    ctx.begin_path();
                                    let _ = ctx.arc(cx, cy, clip.w.min(clip.h) / 2.0, 0.0, std::f64::consts::PI * 2.0);
                                    ctx.set_fill_style(&JsValue::from_str("#ec4899")); // Pink
                                    ctx.fill();
                                }
                                ClipType::Star => {
                                    ctx.set_fill_style(&JsValue::from_str("#eab308")); // Yellow
                                    ctx.fill_rect(clip.x, clip.y, clip.w, clip.h);
                                }
                                ClipType::Text { content } => {
                                    ctx.set_font("bold 28px sans-serif");
                                    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
                                    let _ = ctx.fill_text(content, clip.x, clip.y + 30.0);
                                }
                                ClipType::SolidColor { color } => {
                                    ctx.set_fill_style(&JsValue::from_str(color));
                                    ctx.fill_rect(clip.x, clip.y, clip.w, clip.h);
                                }
                                ClipType::Video { .. } | ClipType::Photo { .. } => {
                                    ctx.set_fill_style(&JsValue::from_str("#3b82f6")); // Blue placeholder
                                    ctx.fill_rect(clip.x, clip.y, clip.w, clip.h);
                                    ctx.set_font("14px sans-serif");
                                    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
                                    let _ = ctx.fill_text(&clip.name, clip.x + 10.0, clip.y + 25.0);
                                }
                                ClipType::Audio { .. } => {
                                    // Audio is represented on audio track
                                }
                            }

                            ctx.restore();
                        }
                    }
                }
            }
        }
    });

    // Transport Actions
    let toggle_play = move |_| {
        set_is_playing.update(|p| *p = !*p);
    };

    let seek_back = move |_| {
        set_playhead_time.update(|t| *t = (*t - 1.0).max(0.0));
    };

    let seek_next = move |_| {
        set_playhead_time.update(|t| *t = *t + 1.0);
    };

    let jump_start = move |_| {
        set_playhead_time.set(0.0);
    };

    // Add Elements / Importers
    let add_element = move |clip_type: ClipType, name: &str| {
        let target_track_type = if matches!(clip_type, ClipType::Audio { .. }) {
            TrackType::Audio
        } else {
            TrackType::Video
        };
        let new_clip = Clip {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            start_time: playhead_time.get(),
            duration: 5.0,
            in_point: 0.0,
            out_point: 5.0,
            clip_type,
            x: 150.0,
            y: 150.0,
            w: 300.0,
            h: 150.0,
            rotation: 0.0,
            opacity: 100.0,
            effects: vec![],
        };

        set_project.update(|proj| {
            if let Some(track) = proj.tracks.iter_mut().find(|t| t.track_type == target_track_type) {
                track.clips.push(new_clip.clone());
            }
            persist_project(proj);
        });
        set_selected_clip_id.set(Some(new_clip.id));
    };

    // Split Clip (Cut / Trim Tool)
    let split_selected_clip = move |_| {
        let current_time = playhead_time.get();
        if let Some(clip_id) = selected_clip_id.get() {
            set_project.update(|proj| {
                for track in &mut proj.tracks {
                    if let Some(pos) = track.clips.iter().position(|c| c.id == clip_id) {
                        let original = track.clips[pos].clone();
                        if current_time > original.start_time && current_time < (original.start_time + original.duration) {
                            let first_duration = current_time - original.start_time;
                            let second_duration = original.duration - first_duration;

                            let mut second_clip = original.clone();
                            second_clip.id = Uuid::new_v4().to_string();
                            second_clip.name = format!("{} (Part 2)", original.name);
                            second_clip.start_time = current_time;
                            second_clip.duration = second_duration;
                            second_clip.in_point = original.in_point + first_duration;

                            track.clips[pos].duration = first_duration;
                            track.clips[pos].out_point = original.in_point + first_duration;

                            track.clips.insert(pos + 1, second_clip);
                            break;
                        }
                    }
                }
                persist_project(proj);
            });
        }
    };

    // Timeline Ruler click handler for Interactive Beat System
    let handle_ruler_click = move |ev: web_sys::MouseEvent| {
        // Assume timeline total width represents 30 seconds
        let timeline_seconds_scale = 30.0;
        let rect = match ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            Some(el) => el.get_bounding_client_rect(),
            None => return,
        };
        if rect.width() <= 0.0 {
            return;
        }
        let click_x = ev.client_x() as f64 - rect.left();
        let approx_time = (click_x / rect.width()) * timeline_seconds_scale;
        
        set_project.update(|proj| {
            proj.beat_markers.push(BeatMarker {
                time: approx_time.max(0.0),
                active: true, // turns bright RED (#FF0000) upon click
            });
            persist_project(proj);
        });
    };

    view! {
        <div class="min-h-screen bg-gray-950 text-gray-100 flex flex-col select-none">
            // Editor Header
            <header class="bg-gray-900 border-b border-gray-800 px-6 py-3 flex justify-between items-center shadow-lg">
                <div class="flex items-center space-x-4">
                    <a href=format!("{}/", crate::config::BASE_PATH) class="bg-gray-800 hover:bg-gray-700 text-indigo-400 px-3 py-1.5 rounded-lg text-sm font-medium transition flex items-center space-x-1">
                        <span>"← Dashboard"</span>
                    </a>
                    <h1 class="text-lg font-bold text-white truncate max-w-md">
                        {move || project.get().title}
                    </h1>
                </div>

                // Transport Controls
                <div class="flex items-center space-x-2 bg-gray-950 px-4 py-1.5 rounded-xl border border-gray-800 shadow">
                    <button on:click=jump_start class="p-2 hover:bg-gray-800 rounded-lg text-gray-300 transition" title="Jump to Start">"⏮"</button>
                    <button on:click=seek_back class="p-2 hover:bg-gray-800 rounded-lg text-gray-300 transition" title="Seek -1s">"⏪"</button>
                    <button
                        on:click=toggle_play
                        class="bg-indigo-600 hover:bg-indigo-500 text-white font-bold px-4 py-1.5 rounded-lg transition shadow flex items-center space-x-1"
                    >
                        <span>{move || if is_playing.get() { "⏸ Pause" } else { "▶ Play" }}</span>
                    </button>
                    <button on:click=seek_next class="p-2 hover:bg-gray-800 rounded-lg text-gray-300 transition" title="Seek +1s">"⏩"</button>
                    <button on:click=split_selected_clip class="bg-purple-600 hover:bg-purple-500 text-white px-3 py-1 rounded-lg text-xs font-bold transition ml-2" title="Split Clip at Playhead">
                        "✂ Split Clip"
                    </button>
                </div>

                <div class="text-xs font-mono bg-gray-950 px-3 py-1.5 rounded-lg border border-gray-800 text-indigo-400">
                    "Time: " {move || format!("{:05.2}s", playhead_time.get())}
                </div>
            </header>

            // Main Editor Body: Canvas Preview + Importers & Inspector
            <div class="flex-1 grid grid-cols-1 lg:grid-cols-4 gap-4 p-4 overflow-hidden">
                // Left & Center: Canvas Preview Screen & Importers
                <div class="lg:col-span-3 flex flex-col space-y-4">
                    // Canvas Preview Screen
                    <div class="flex-1 bg-gray-900 border border-gray-800 rounded-2xl flex items-center justify-center relative overflow-hidden shadow-2xl min-h-[420px]">
                        <canvas
                            node_ref=canvas_ref
                            width="960"
                            height="540"
                            class="max-w-full max-h-[500px] object-contain rounded-lg shadow-xl border border-gray-800"
                        ></canvas>
                        <div class="absolute top-4 left-4 bg-black/60 backdrop-blur-md px-3 py-1 rounded-md text-xs font-mono text-gray-300 border border-white/10">
                            {move || project.get().aspect_ratio.to_string()} " • " {move || project.get().resolution.label()}
                        </div>
                    </div>

                    // Track Media & Element Importers Toolbar
                    <div class="bg-gray-900 border border-gray-800 rounded-2xl p-4 flex flex-wrap items-center justify-between gap-3 shadow-lg">
                        <div class="flex items-center space-x-2">
                            <span class="text-xs font-bold uppercase text-gray-400">"Importers & Elements:"</span>
                            <button
                                on:click=move |_| add_element(ClipType::Video { src: "sample.mp4".to_string() }, "Video Clip")
                                class="bg-gray-800 hover:bg-gray-700 text-indigo-300 px-3 py-1.5 rounded-lg text-xs font-semibold border border-indigo-500/30 transition"
                            >
                                "+ Video / Photo"
                            </button>
                            <button
                                on:click=move |_| add_element(ClipType::Audio { src: "track.mp3".to_string(), waveform: vec![0.2, 0.8, 0.5] }, "Audio Clip")
                                class="bg-gray-800 hover:bg-gray-700 text-emerald-300 px-3 py-1.5 rounded-lg text-xs font-semibold border border-emerald-500/30 transition"
                            >
                                "+ Audio"
                            </button>
                            <button
                                on:click=move |_| add_element(ClipType::Rectangle, "Rectangle Shape")
                                class="bg-gray-800 hover:bg-gray-700 text-pink-300 px-3 py-1.5 rounded-lg text-xs font-semibold border border-pink-500/30 transition"
                            >
                                "+ Rectangle"
                            </button>
                            <button
                                on:click=move |_| add_element(ClipType::Circle, "Circle Shape")
                                class="bg-gray-800 hover:bg-gray-700 text-yellow-300 px-3 py-1.5 rounded-lg text-xs font-semibold border border-yellow-500/30 transition"
                            >
                                "+ Circle"
                            </button>
                            <button
                                on:click=move |_| add_element(ClipType::Text { content: "New Text Layer".to_string() }, "Text Layer")
                                class="bg-gray-800 hover:bg-gray-700 text-blue-300 px-3 py-1.5 rounded-lg text-xs font-semibold border border-blue-500/30 transition"
                            >
                                "+ Text Layer"
                            </button>
                        </div>
                    </div>
                </div>

                // Right Panel: Layer Effect Inspector & Solid Color Tool
                <div class="bg-gray-900 border border-gray-800 rounded-2xl p-4 flex flex-col space-y-4 shadow-xl overflow-y-auto">
                    <h3 class="text-sm font-bold text-white border-b border-gray-800 pb-2">"Layer Effect Inspector"</h3>

                    {move || {
                        let proj = project.get();
                        let selected_id = selected_clip_id.get();
                        let mut found_clip = None;

                        for track in &proj.tracks {
                            for clip in &track.clips {
                                if Some(clip.id.clone()) == selected_id {
                                    found_clip = Some(clip.clone());
                                    break;
                                }
                            }
                        }

                        if let Some(clip) = found_clip {
                            let clip_id_color = clip.id.clone();
                            let clip_id_x = clip.id.clone();
                            let clip_id_y = clip.id.clone();
                            let clip_id_w = clip.id.clone();
                            let clip_id_h = clip.id.clone();
                            let clip_id_eff = clip.id.clone();

                            view! {
                                <div class="space-y-4 text-xs">
                                    <div class="bg-gray-950 p-3 rounded-xl border border-gray-800">
                                        <div class="font-bold text-indigo-400 text-sm mb-1">{clip.name}</div>
                                        <div class="text-gray-500 font-mono">"ID: " {clip.id.chars().take(8).collect::<String>()}</div>
                                    </div>

                                    // Solid Color Conversion Tool
                                    <div class="space-y-2 bg-gray-950 p-3 rounded-xl border border-gray-800">
                                        <span class="font-semibold text-gray-300">"Solid Color Fill Layer Tool"</span>
                                        <div class="flex items-center space-x-2">
                                            <input
                                                type="color"
                                                value="#6366f1"
                                                on:input=move |ev| {
                                                    let color_hex = event_target_value(&ev);
                                                    set_project.update(|p| {
                                                        for t in &mut p.tracks {
                                                            for c in &mut t.clips {
                                                                if c.id == clip_id_color {
                                                                    c.clip_type = ClipType::SolidColor { color: color_hex.clone() };
                                                                }
                                                            }
                                                        }
                                                        persist_project(p);
                                                    });
                                                }
                                                class="w-10 h-8 bg-transparent cursor-pointer rounded"
                                            />
                                            <span class="text-gray-400">"Convert to Solid Color Fill"</span>
                                        </div>
                                    </div>

                                    // Transform Properties
                                    <div class="space-y-2 bg-gray-950 p-3 rounded-xl border border-gray-800">
                                        <span class="font-semibold text-gray-300">"Transform Properties"</span>
                                        <div class="grid grid-cols-2 gap-2">
                                            <div>
                                                <label class="text-gray-400">"X Pos"</label>
                                                <input
                                                    type="number"
                                                    prop:value=clip.x
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev).parse().unwrap_or(0.0);
                                                        set_project.update(|p| {
                                                            for t in &mut p.tracks { for c in &mut t.clips { if c.id == clip_id_x { c.x = val; } } }
                                                            persist_project(p);
                                                        });
                                                    }
                                                    class="w-full bg-gray-900 border border-gray-800 rounded px-2 py-1 text-white"
                                                />
                                            </div>
                                            <div>
                                                <label class="text-gray-400">"Y Pos"</label>
                                                <input
                                                    type="number"
                                                    prop:value=clip.y
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev).parse().unwrap_or(0.0);
                                                        set_project.update(|p| {
                                                            for t in &mut p.tracks { for c in &mut t.clips { if c.id == clip_id_y { c.y = val; } } }
                                                            persist_project(p);
                                                        });
                                                    }
                                                    class="w-full bg-gray-900 border border-gray-800 rounded px-2 py-1 text-white"
                                                />
                                            </div>
                                            <div>
                                                <label class="text-gray-400">"Width"</label>
                                                <input
                                                    type="number"
                                                    prop:value=clip.w
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev).parse().unwrap_or(100.0);
                                                        set_project.update(|p| {
                                                            for t in &mut p.tracks { for c in &mut t.clips { if c.id == clip_id_w { c.w = val; } } }
                                                            persist_project(p);
                                                        });
                                                    }
                                                    class="w-full bg-gray-900 border border-gray-800 rounded px-2 py-1 text-white"
                                                />
                                            </div>
                                            <div>
                                                <label class="text-gray-400">"Height"</label>
                                                <input
                                                    type="number"
                                                    prop:value=clip.h
                                                    on:input=move |ev| {
                                                        let val = event_target_value(&ev).parse().unwrap_or(100.0);
                                                        set_project.update(|p| {
                                                            for t in &mut p.tracks { for c in &mut t.clips { if c.id == clip_id_h { c.h = val; } } }
                                                            persist_project(p);
                                                        });
                                                    }
                                                    class="w-full bg-gray-900 border border-gray-800 rounded px-2 py-1 text-white"
                                                />
                                            </div>
                                        </div>
                                    </div>

                                    // Attached Modular Effects
                                    <div class="space-y-2 bg-gray-950 p-3 rounded-xl border border-gray-800">
                                        <div class="flex justify-between items-center">
                                            <span class="font-semibold text-gray-300">"Modular Effects"</span>
                                            <button
                                                on:click=move |_| {
                                                    set_project.update(|p| {
                                                        for t in &mut p.tracks {
                                                            for c in &mut t.clips {
                                                                if c.id == clip_id_eff {
                                                                    c.effects.push(AppliedEffect {
                                                                        effect_id: "flash_invert_v1".to_string(),
                                                                        enabled: true,
                                                                        params: [("intensity".to_string(), 1.0)].into(),
                                                                    });
                                                                }
                                                            }
                                                        }
                                                        persist_project(p);
                                                    });
                                                }
                                                class="bg-indigo-600 hover:bg-indigo-500 text-white px-2 py-1 rounded text-xs font-bold"
                                            >
                                                "+ Attach Effect"
                                            </button>
                                        </div>
                                        <div class="space-y-2 mt-2">
                                            {clip.effects.into_iter().map(|eff| {
                                                view! {
                                                    <div class="bg-gray-900 p-2 rounded border border-gray-800 flex justify-between items-center">
                                                        <span>{eff.effect_id}</span>
                                                        <span class="text-emerald-400 font-semibold">"Active"</span>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            view! {
                                <div class="text-gray-500 text-center py-12">
                                    "Select a clip on the timeline to inspect and edit its properties and modular effects."
                                </div>
                            }
                        }
                    }}
                </div>
            </div>

            // Bottom Section: Timeline Ruler & Interactive Beat System & Tracks
            <div class="bg-gray-900 border-t border-gray-800 p-4 shadow-2xl">
                <div class="flex justify-between items-center mb-2">
                    <span class="text-xs font-bold uppercase tracking-wider text-gray-400">"Timeline & Interactive Beat System"</span>
                    <span class="text-xs text-indigo-400">"Click top timeline ruler to drop RED (#FF0000) beat marker cue node"</span>
                </div>

                // Timeline Ruler with Clickable Beat Markers (Z-Index Positioning rendered on top)
                <div
                    on:click=handle_ruler_click
                    class="h-10 bg-gray-950 border border-gray-800 rounded-lg relative cursor-pointer overflow-hidden flex items-center px-4 shadow-inner"
                >
                    <div class="absolute inset-x-0 top-0 bottom-0 flex justify-between text-[10px] font-mono text-gray-600 px-2 pointer-events-none items-center">
                        <span>"00:00.00"</span>
                        <span>"00:05.00"</span>
                        <span>"00:10.00"</span>
                        <span>"00:15.00"</span>
                        <span>"00:20.00"</span>
                        <span>"00:25.00"</span>
                        <span>"00:30.00"</span>
                    </div>

                    // Render Beat Markers (z-index top, bright RED #FF0000)
                    {move || {
                        project.get().beat_markers.into_iter().map(|bm| {
                            let left_pct = (bm.time / 30.0) * 100.0;
                            view! {
                                <div
                                    style=format!("left: {}%;", left_pct)
                                    class="absolute top-0 bottom-0 w-1 bg-[#FF0000] z-50 shadow-[0_0_10px_#FF0000]"
                                    title=format!("Beat Marker at {:.2}s", bm.time)
                                >
                                    <div class="w-3 h-3 bg-[#FF0000] rounded-full -ml-1 shadow-[0_0_12px_#FF0000]"></div>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}

                    // Playhead indicator on ruler
                    <div
                        style=move || format!("left: {}%;", (playhead_time.get() / 30.0) * 100.0)
                        class="absolute top-0 bottom-0 w-0.5 bg-indigo-500 z-40 pointer-events-none"
                    ></div>
                </div>

                // Tracks & Clips Representation
                <div class="mt-3 space-y-2 max-h-[180px] overflow-y-auto">
                    {move || {
                        project.get().tracks.into_iter().map(|track| {
                            let track_clips = track.clips;
                            view! {
                                <div class="bg-gray-950 border border-gray-800 rounded-xl p-2.5 flex items-center space-x-3">
                                    <div class="w-32 text-xs font-bold text-gray-300 truncate">{track.name}</div>
                                    <div class="flex-1 h-10 bg-gray-900 rounded-lg relative border border-gray-800 overflow-hidden">
                                        {track_clips.into_iter().map(|clip| {
                                            let clip_id_sel = clip.id.clone();
                                            let left_pct = (clip.start_time / 30.0) * 100.0;
                                            let width_pct = (clip.duration / 30.0) * 100.0;
                                            view! {
                                                <div
                                                    on:click=move |_| set_selected_clip_id.set(Some(clip_id_sel.clone()))
                                                    style=format!("left: {}%; width: {}%;", left_pct, width_pct)
                                                    class="absolute top-1 bottom-1 bg-indigo-600/80 hover:bg-indigo-500 border border-indigo-400/50 rounded-md px-2 py-1 text-xs text-white font-medium flex items-center justify-between cursor-pointer truncate shadow transition"
                                                >
                                                    <span class="truncate">{clip.name}</span>
                                                    <span class="text-[10px] text-indigo-200 font-mono">"In/Out"</span>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}
