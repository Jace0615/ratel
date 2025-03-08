#![allow(non_snake_case)]
use dioxus::prelude::*;
// use dioxus_popup::PopupZone;
use dioxus_translate::*;

use super::components::*;
use crate::{components::footer::Footer, route::Route};
use by_components::loaders::cube_loader::CubeLoader;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    #[cfg(feature = "web")]
    let mut scroll_position = use_signal(|| 0.0);
    let selected = use_memo(move || {
        #[cfg(feature = "web")]
        {
            let y = scroll_position();
            let height = match web_sys::window() {
                Some(window) => window
                    .inner_height()
                    .unwrap_or_default()
                    .as_f64()
                    .unwrap_or_default(),
                None => 0.0,
            };

            #[cfg(not(feature = "web"))]
            let height = 0.0;

            let i = if y < height * 0.7 {
                0
            } else if y < height * 1.7 {
                1
            } else if y < height * 2.7 {
                2
            } else if y < height * 3.7 {
                3
            } else if y <= height * 4.1 {
                4
            } else {
                5
            };

            return i;
        }
        #[cfg(not(feature = "web"))]
        0
    });

    #[cfg(feature = "web")]
    let _ = use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let script = r#"
            window.addEventListener('scroll', () => {
                dioxus.send(`${window.scrollY}`);
            });
        "#;
        let mut eval = document::eval(script);

        loop {
            let y = eval
                .recv::<String>()
                .await
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or_default();
            scroll_position.set(y);
        }
    });

    rsx! {
        div { class: "w-full h-full bg-background text-white",
            Header { lang, selected: selected() }
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                        CubeLoader {}
                    }
                },
            }
            div { class: "w-full overflow-x-hidden scroll-smooth", Outlet::<Route> {} }

            div { class: "max-w-[1440px] w-full",
                Footer { lang }
            }

        }
        if selected() != 5 {
            BottomSheet {
                onclick: move |_| {
                    let height = match web_sys::window() {
                        Some(window) => {
                            window.inner_height().unwrap_or_default().as_f64().unwrap_or_default()
                        }
                        None => 0.0,
                    };
                    let next_position = height * (selected() + 1) as f64;
                    let script = format!(
                        "window.scrollTo({{ top: {next_position}, behavior: 'smooth' }});",
                    );
                    let _ = document::eval(&script);
                },
            }
        }
    }
}
