use leptos::{ev::MouseEvent, prelude::*};
use leptos_icons::Icon;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::theme_toggle::ThemeToggle;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

fn class_with_dark_mode(is_dark_mode: RwSignal<bool>, base: &'static str) -> impl Fn() -> String {
    move || {
        if is_dark_mode.get() {
            format!("{} dark-mode", base)
        } else {
            base.to_string()
        }
    }
}

#[component]
fn LandingPage() -> impl IntoView {
    provide_meta_context();

    let icon = RwSignal::new(true);
    let toggle_theme = Box::new(move |_: MouseEvent| icon.update(|dark| *dark = !*dark));

    view! {
        <Meta name="color-scheme" content=move || if icon.get() { "light".to_string() } else { "dark".to_string() } />
        <div class=class_with_dark_mode(icon, "gradient-background")>
            // <img src="/floral.png" alt="Decorative Image 1" class=class_with_dark_mode(icon, "background-image image-1") />
            // <img src="/floral2.png" alt="Decorative Image 2" class=class_with_dark_mode(icon, "background-image image-2") />
            <img src="/bookshelf.png" alt="Decorative Image 1" class=class_with_dark_mode(icon, "background-image image-1") />
            <div class=class_with_dark_mode(icon, "image-overlay") />

            <nav class="navbar">
                <div class="toggle-container">
                    <ThemeToggle dark=icon toggle_theme=toggle_theme />
                </div>
                <ul>
                    <li><a href="#home">"Home"</a></li>
                    <li><a href="#about">"About"</a></li>
                    <li><a href="#contact">"Contact"</a></li>
                </ul>
            </nav>
            <div class=class_with_dark_mode(icon, "content-box")>
                <h1>"Welcome to My Webapp"</h1>
                <p>"This is a simple landing page built with Leptos."</p>
                // <div class="frame">
                //     <img src="/image0.jpg" width="400px" height="300px" />
                // </div>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/wedding-leptos.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=LandingPage/>
                    // <Route path="/admin" view=AdminPanel/>
                </Routes>
            </main>
        </Router>
    }
}
