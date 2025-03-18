#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::components::icons;

use super::*;

#[component]
pub fn Community(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tr: CommunityTranslate = translate(&lang);

    rsx! {
        div {
            id: "community",
            class: "w-full max-w-1177 h-screen flex flex-col items-start justify-center gap-50 max-[1177px]:mx-10",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
            }

            Tabs {
                tabs: vec![tr.tab_legislation.to_string(), tr.tab_discussion.to_string()],
                ontab: |i| {
                    tracing::debug!("Tab selected: {}", i);
                },
            }

            ComingSoon { class: "w-full h-full max-h-430" }
        }
    }
}

#[component]
pub fn ComingSoon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {..attributes,
            div { class: "w-full h-full bg-bg flex flex-col items-center justify-center rounded-[20px] gap-30",
                icons::ComingSoon {}
                p { class: "text-5xl font-bold text-text-primary", "Coming soon" }
            }
        }
    }
}

#[component]
pub fn Tabs(tabs: Vec<String>, ontab: EventHandler<usize>) -> Element {
    let mut selected = use_signal(|| 0);

    rsx! {
        div { class: "w-full flex flex-row items-center justify-center",
            div { class: "flex flex-row items-center jsutify-center gap-20 rounded-full overflow-hidden bg-bg",
                for (i , name) in tabs.iter().enumerate() {
                    button {
                        class: "px-30 py-18 text-[15px]/16 font-bold text-secondary hover:text-hover cursor-pointer rounded-full",
                        color: if selected() == i { "var(--color-text-primary)" },
                        background: if selected() == i { "var(--color-tab-hover)" },
                        onclick: move |_| {
                            ontab(i);
                            selected.set(i);
                        },
                        "{name}"
                    }
                }
            }
        }
    }
}

translate! {
    CommunityTranslate;

    title: {
        ko: "Community",
        en: "Community",
    }

    mission: {
        ko: "Ratel DAO: 탈중앙화된 거버넌스 허브",
        en: "Ratel DAO: Decentralized Governance Hub",
    }

    description: {
        ko: "Ratel DAO에 가입하여 완전히 탈중앙화되고 투명한 생태계에서 기여하고 투표하며 주요 결정에 영향을 미치세요.",
        en: "Join Ratel DAO to contribute, vote, and influence major decisions in a fully decentralized and transparent ecosystem.",
    }

    tab_legislation: {
        ko: "법안",
        en: "Legislation Proposal",
    }

    tab_discussion: {
        ko: "토론",
        en: "Discussion",
    }
}
