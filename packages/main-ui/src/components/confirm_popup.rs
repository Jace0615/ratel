#![allow(non_snake_case)]
use crate::components::button::primary_button::PrimaryButton;
use crate::components::icons::CharacterWithCircle;

use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn ConfirmPopup(
    #[props(default ="welcome_popup".to_string())] id: String,
    lang: Language,
    title: String,
    description: String,
    btn_label: String,
) -> Element {
    let mut popup: PopupService = use_context();

    rsx! {
        div { id, class: "max-w-390 w-full",
            div { class: "w-full flex flex-col gap-35",
                WelcomeHeader { lang, title, description }

                PrimaryButton {
                    width: "100%",
                    onclick: move |_| {
                        popup.close();
                    },
                    {btn_label}
                }
            }

            SigninPopupFooter { lang }
        }
    }
}

#[component]
pub fn WelcomeHeader(lang: Language, title: String, description: String) -> Element {
    rsx! {
        div { class: "w-full flex flex-col gap-24 items-center justify-center mt-35",
            p { class: "text-white font-bold text-2xl max-[900px]:!text-[22px]", {title} }
            div { class: "block max-[900px]:!hidden",
                CharacterWithCircle { size: 100 }
            }
            p { class: "text-neutral-400 text-center text-base font-medium max-[900px]:!text-[15px]",
                {description}
            }
        }
    }
}

#[component]
pub fn SigninPopupFooter(lang: Language) -> Element {
    let tr = translate::<SigninPopupFooterTranslate>(&lang);
    rsx! {
        // TODO: applying policy and terms.
        div { class: "flex flex-row gap-10 mt-35 justify-center",
            button {
                class: "cursor-pointer",
                onclick: move |_| {
                    tracing::debug!("Privacy policy clicked");
                },
                span { class: "text-neutral-400 text-xs/14 font-medium", {tr.privacy_policy} }
            }
            button {
                class: "cursor-pointer",
                onclick: move |_| {
                    tracing::debug!("Term of service clicked");
                },
                span { class: "text-neutral-400 text-xs/14 font-medium", {tr.term_of_service} }
            }
        }
    }
}

translate! {
    SigninPopupFooterTranslate;

    privacy_policy: {
        ko: "개인정보 처리방침",
        en: "Privacy Policy",
    },

    term_of_service: {
        ko: "이용약관",
        en: "Term of Service",
    },
}
