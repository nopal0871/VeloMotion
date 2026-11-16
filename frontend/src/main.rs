<<<<<<< HEAD
mod config;
mod effects;
mod models;
mod storage;
=======
>>>>>>> 3c3702960cb7d26ba2cdfe0d5e737ed02cd3374f
mod views;

use leptos::*;
use leptos_router::*;
<<<<<<< HEAD
use views::dashboard::DashboardPage as DashboardView;
use views::editor::EditorPage as EditorView;
=======
use views::dashboard::DashboardView;
use views::editor::EditorView;
>>>>>>> 3c3702960cb7d26ba2cdfe0d5e737ed02cd3374f

#[component]
fn App() -> impl IntoView {
    // Set panic hook for better WASM debugging
    console_error_panic_hook::set_once();

    view! {
<<<<<<< HEAD
        <Router base=config::BASE_PATH>
=======
        <Router>
>>>>>>> 3c3702960cb7d26ba2cdfe0d5e737ed02cd3374f
            <main class="w-full min-h-screen bg-zinc-950 text-zinc-100">
                <Routes>
                    <Route path="" view=DashboardView/>
                    <Route path="/editor" view=EditorView/>
                    <Route path="/editor/:id" view=EditorView/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
