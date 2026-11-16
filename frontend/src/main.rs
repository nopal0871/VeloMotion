mod views;

use leptos::*;
use leptos_router::*;
use views::dashboard::DashboardView;
use views::editor::EditorView;

#[component]
fn App() -> impl IntoView {
    // Set panic hook for better WASM debugging
    console_error_panic_hook::set_once();

    view! {
        <Router>
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
