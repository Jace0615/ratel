#![allow(non_snake_case)]
use super::{SignupPopup, Socials};
use crate::components::{
    button::{primary_button::PrimaryButton, secondary_botton::SecondaryButton},
    icons::CharacterSymbol,
};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn Top(#[props(default ="".to_string())] class: String, lang: Language) -> Element {
    rsx! {
        div { class: "hidden md:!block",
            DesktopTop { lang }
        }
        div { class: "block md:!hidden",
            MobileTop { lang }
        }
    }
}

#[component]
pub fn DesktopTop(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let tr: TopTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    rsx! {
        div {
            id: "top",
            class: "w-screen h-screen flex flex-col items-center justify-center gap-100",
            ..attributes,
            div { class: "flex flex-col items-center justify-center gap-32",
                CharacterSymbol {}
                h1 { class: "text-5xl/56 text-center font-bold text-white whitespace-pre-line",
                    {tr.slogan}
                }
                p { class: "text-lg text-center font-normal text-c-wg-30 whitespace-pre-line",
                    {tr.description}
                }

                Socials { class: "flex flex-row gap-50" }
            }

            div { class: "flex flex-row gap-20",
                // TODO: implement downloading whitepaper
                PrimaryButton { onclick: |_| {}, {tr.btn_learn} }

                // TODO: implement Sign in
                SecondaryButton {
                    onclick: move |_| {
                        tracing::debug!("Learn more clicked");
                        popup.open(rsx! {
                            SignupPopup { class: "w-460", lang: lang.clone() }
                        });
                    },
                    {tr.btn_join}
                }
            }
        }
    }
}

#[component]
pub fn MobileTop(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let tr: TopTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    rsx! {
        div {
            id: "top",
            class: "w-screen h-full flex flex-col items-center justify-start",
            ..attributes,
            div { class: "flex flex-col items-center justify-center gap-[32px]",
                div { class: "w-[156px] h-[128px] flex items-center justify-center ",
                    CharacterSymbol {}
                }
                p { class: "text-[28px] text-center font-bold leading-[56px] text-white whitespace-pre-line",
                    {tr.slogan}
                }
                p { class: "text-[14px] text-center font-normal text-[#AEAEAE] whitespace-pre-line",
                    {tr.description_mobile}
                }

                Socials { class: "flex flex-row gap-50 mb-[115px]" }
            }

            div { class: "w-full flex flex-col justify-center items-center gap-[16px] ",
                // TODO: implement downloading whitepaper
                div { class: "w-[291px] h-[48px] flex justify-center items-center rounded-[10px] whitespace-nowrap bg-[#FCB300] p-[20px]",
                    PrimaryButton {
                        onclick: |_| {},
                        class: "w-full font-bold text-[15px] text-black",
                        {tr.btn_learn}
                    }
                }
                // TODO: implement Sign in
                div { class: "w-[291px] h-[48px] flex justify-center items-center rounded-[10px] whitespace-nowrap bg-[#ffffff] p-[20px] font-bold text-[15px] text-black",
                    SecondaryButton {
                        onclick: move |_| {
                            tracing::debug!("Learn more clicked");
                            popup.open(rsx! {
                                SignupPopup { class: "w-[460px]", lang: lang.clone() }
                            });
                        },
                        {tr.btn_join}
                    }
                }
            }
        }
    }
}

translate! {
    TopTranslate;

    slogan: {
        ko: "공정한 암호화폐 법안을 위해\n함께 싸워요!",
        en: "Join the Fight\nfor Fair Crypto Legislation",
    },
    description: {
        ko: "한국 시민과 의원을 연결하는 첫 번째 플랫폼으로\n암호화폐 산업을 위한 제도 개혁을 추진합니다. 함께 하실래요?",
        en: "The first platform connecting South Korea’s citizens with lawmakers to drive\ninstitutional reform for the crypto industry. Are you with us?",
    },

    description_mobile: {
        ko: "한국 시민과 의원을 연결하는 첫 번째 플랫폼으로\n암호화폐 산업을 위한 제도 개혁을 추진합니다. 함께 하실래요?",
        en: "The first platform connecting South Korea’s citizens with \nlawmakers to drive institutional reform for the crypto industry. Are you with us?",
    },

    btn_learn: {
        ko: "$RATEL에 대해 더 알아보기",
        en: "LEARN MORE ABOUT $RATEL",
    },
    btn_join: {
        ko: "지금 참여하기",
        en: "JOIN THE MOVEMENT",
    },
}
