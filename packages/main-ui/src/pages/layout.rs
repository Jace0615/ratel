#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupZone;
use dioxus_translate::*;

use super::header::Header;
use crate::{components::footer::Footer, route::Route, theme::Theme};

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let theme: Theme = use_context();
    let theme = theme.get_data();

    rsx! {
        div {
            class: "flex flex-col items-center justify-start w-full min-h-[100vh] text-white max-[1440px]:px-[10px]",
            style: "background: {theme.background}",
            div { class: "max-w-[1440px] w-full",
                Header { lang }
            }
            div { class: "w-full max-w-[1440px] flex-1",
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        div { class: "w-full h-full flex items-center justify-center text-white", "Loading..." }
                    },
                    Outlet::<Route> {}
                }
            }
            div { class: "max-w-[1440px] w-full",
                Footer { lang }
            }
        }
        PopupZone {}
    }
}
