#![allow(non_snake_case)]
use crate::pages::components::SectionHeader;

use super::controller::*;
use super::i18n::*;
use bdk::prelude::*;

#[component]
pub fn PoliticiansPage(lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: PoliticiansTranslate = translate(&lang);
    let politicians = ctrl.politicians()?;

    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }


        div { class: "flex flex-col gap-50 w-full",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
            }

            for p in politicians.items.iter() {
                "{p:?}"
            }
        }
    }
}
