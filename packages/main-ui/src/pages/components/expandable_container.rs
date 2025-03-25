#![allow(non_snake_case, dead_code, unused_variables)]
use bdk::prelude::*;

#[component]
pub fn ExpandableContainer(
    tag: String,
    total_count: i64,
    icon: Element,
    expanded: bool,
    onclick: EventHandler<()>,
    children: Element,
    #[props(default ="text-c-c-20".to_string())] text_color: String,
) -> Element {
    let rotate = if expanded { "rotate-0" } else { "rotate-270" };
    let grow = if expanded { "grow w-full" } else { "" };

    let outer = if expanded { "w-full" } else { "w-fit" };
    let tag_style = if expanded {
        "justify-start"
    } else {
        "justify-center rotate-270"
    };
    let icon_style = if expanded { "rotate-0" } else { "rotate-270" };
    let children_style = if expanded {
        "w-full"
    } else {
        "overflow-hidden w-0 h-0"
    };
    let header_style = if expanded {
        "flex-row"
    } else {
        "h-full flex-col"
    };

    rsx! {
        div {
            class: "transition-[width] duration-300  items-center justify-start h-full bg-bg rounded-[20px] cursor-pointer px-30 py-40 flex flex-col {text_color} {outer} gap-40",
            onclick: move |_| {
                tracing::debug!("clicked");
                onclick(());
            },
            div { class: "transition-all w-full flex {header_style} items-center justify-between gap-10",
                if expanded {
                    span { class: "font-bold text-[32px]/22", "{total_count}" }
                    p { class: "grow text-white text-xl/22 font-bold flex flex-row items-center whitespace-nowrap {tag_style}",
                        {tag}
                    }
                    {icon}
                } else {
                    {icon}
                    p { class: "grow text-white text-xl/22 font-bold flex flex-row items-center whitespace-nowrap {tag_style}",
                        {tag}
                    }
                    span { class: "font-bold text-[32px]/22", "{total_count}" }
                }
            }

            div { class: children_style, {children} }
        }
    }
}

#[component]
pub fn MobileExpandableContainer(
    tag: String,
    total_count: i64,
    icon: Element,
    onclick: EventHandler<()>,
    children: Element,
    #[props(default ="text-c-c-20".to_string())] text_color: String,
) -> Element {
    rsx! {
        div { class: "w-full h-full flex flex-col justify-start items-center gap-[20px] {text_color}",
            div { class: "w-full flex items-center justify-between gap-[24px]",
                div { class: "w-full flex flex-row gap-[10px] justify-start items-center",
                    {icon}
                    p { class: "grow text-white text-[20px] font-bold flex flex-row items-center whitespace-nowrap",
                        {tag}
                    }
                }
                span { class: "text-[20px] font-bold", "{total_count}" }
            
            }

            div { class: "w-full", {children} }
        }
    }
}
