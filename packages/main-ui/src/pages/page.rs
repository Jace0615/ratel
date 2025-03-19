#![allow(non_snake_case)]
use bdk::prelude::*;
use by_components::meta::MetaPage;
use subscription::Subscription;

use super::components::*;

#[component]
pub fn HomePage(lang: Language) -> Element {
    let tr: TopTranslate = translate(&lang);
    let image = asset!("/public/logos/logo.png");

    rsx! {
        MetaPage { title: "Ratel", description: tr.description, image: "{image}" }
        div { class: "flex flex-col w-full justify-start items-center",
            div { class: "hidden md:!block",
                Top { lang }
                About { lang }
                PoliticianStance { lang }
                Community { lang }
                Support { lang }
                Subscription { lang }
                Footer { lang }
            }
            div { class: "block md:!hidden",
                Top { lang }
            }
        }
    }
}
