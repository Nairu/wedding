use leptos::{ev::MouseEvent, prelude::*};
use leptos_icons::*;
use leptos_meta::Meta;

#[component]
pub fn ThemeToggle(
    dark: RwSignal<bool>,
    toggle_theme: Box<dyn FnMut(MouseEvent)>,
) -> impl IntoView {
    // Dynamically select the icon based on the theme
    let icon = Signal::derive(move || {
        if dark.get() {
            icondata::BsMoonStars
        } else {
            icondata::BsSun
        }
    });

    // let div_style = view! {
    //     <{..} style="display: flex; align-items: center; justify-content: center; width: 100vw; height: 100vh; margin: 0; cursor: pointer;" />
    // };

    // let icon_style = view! {
    //     <{..} style="padding: 0.5rem; border: 4px solid; border-radius: 1rem;" />
    // };

    let icon_style = view! {
        <{..} style="transition: color 0.3s ease; position: absolute; left: 7.5px; top: 7.5px" color=move || if dark.get() { "#f0f0f0" } else { "#333" }/>
    };

    view! {
        // Meta tag for color scheme
        <Meta name="color-scheme" content=move || if dark.get() { "dark".to_string() } else { "light".to_string() } />

        // Theme toggle container
        <div class=move || if dark.get() { "theme-toggle dark" } else { "theme-toggle" } on:click=toggle_theme>
            // Dynamic Icon
            <Icon icon=icon width="2rem" height="2rem" {..icon_style}/>
        </div>
    }
}
