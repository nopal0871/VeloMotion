use leptos::*;
use leptos_router::*;
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
        </div>
    }
}
