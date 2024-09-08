use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let rand = RwSignal::new(0);
    let on_click = move |_| {
        spawn_local(async move {
            match api::random_number().await {
                Ok(number) => rand.set(number),
                Err(e) => leptos::logging::error!("{e}")
            }
        });
    };

    view! {
        <button on:click=on_click>"Click Me"</button>
        <p>{rand}</p>
    }
}
