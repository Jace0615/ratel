#![allow(non_snake_case)]
use bdk::prelude::*;
use dioxus_popup::PopupService;
use dto::{Need, SupportSubmitRequest};

use crate::{
    components::{
        button::secondary_botton::SecondaryButton,
        confirm_popup::ConfirmPopup,
        dropdown::{Dropdown, MobileDropdown},
        icons::BackgroundTriangle,
    },
    config,
};

use super::*;

#[component]
pub fn Support(lang: Language) -> Element {
    rsx! {
        div { class: "hidden md:!block",
            DesktopSupport { lang }
        }
        div { class: "block md:!hidden",
            MobileSupport { lang }
        }
    }
}

#[component]
pub fn DesktopSupport(lang: Language) -> Element {
    let mut popup: PopupService = use_context();
    let tr: SupportTranslate = translate(&lang);
    let mut req = use_signal(|| SupportSubmitRequest::default());

    rsx! {
        div {
            id: "support",
            class: "w-full h-screen flex flex-col items-center justify-center",
            div { class: "max-w-1177 grid grid-cols-2 gap-50 max-[1177px]:mx-10",
                div { class: "col-span-1 w-full",
                    SectionHeader {
                        section_name: tr.title,
                        title: tr.mission,
                        description: tr.description,
                        with_line: false,
                    }
                }

                div { class: "col-span-1 w-full flex flex-col items-start gap-50",
                    div { class: "col-span-1 w-full flex flex-col items-start gap-30",
                        div { class: "w-full grid grid-cols-2 gap-24 max-600:grid-cols-1",
                            LabeledInput {
                                label_name: tr.label_first_name,
                                placeholder: tr.placeholder_first_name,
                                oninput: move |e| {
                                    req.with_mut(move |r| r.first_name = e);
                                },
                            }
                            LabeledInput {
                                label_name: tr.label_last_name,
                                placeholder: tr.placeholder_last_name,
                                oninput: move |e| {
                                    req.with_mut(move |r| r.last_name = e);
                                },
                            }
                        }

                        LabeledInput {
                            class: "w-full",
                            label_name: tr.label_email,
                            placeholder: tr.placeholder_email,
                            oninput: move |e| {
                                req.with_mut(move |r| r.email = e);
                            },
                        }

                        LabeledInput {
                            class: "w-full",
                            label_name: tr.label_company,
                            placeholder: tr.placeholder_company,
                            oninput: move |e| {
                                req.with_mut(move |r| r.company_name = e);
                            },
                        }

                        Labeled { class: "w-full", label_name: tr.label_needs,
                            Dropdown {
                                class: "w-full h-50",
                                items: Need::variants(&lang),
                                onselect: move |value: String| {
                                    req.with_mut(move |r| r.needs = value.parse().unwrap_or_default());
                                },
                            }
                        }


                        LabeledInput {
                            class: "w-full",
                            label_name: tr.label_help,
                            placeholder: tr.placeholder_help,
                            oninput: move |e| {
                                req.with_mut(move |r| r.help = e);
                            },
                        }
                    } // end of form

                    SecondaryButton {
                        onclick: move |_| async move {
                            let endpoint = config::get().main_api_endpoint;
                            let SupportSubmitRequest {
                                first_name,
                                last_name,
                                email,
                                company_name,
                                needs,
                                help,
                            } = req();
                            match dto::Support::get_client(endpoint)
                                .submit(first_name, last_name, email, company_name, needs, help)
                                .await
                            {
                                Ok(_) => {
                                    btracing::info!("Thank you for your submission!");
                                    let tr: InquiryTranslate = translate(&lang);
                                    popup.open(rsx! {
                                        ConfirmPopup {
                                            lang,
                                            title: tr.title,
                                            description: tr.description,
                                            btn_label: tr.btn_label,
                                        }
                                    });
                                }
                                Err(e) => {
                                    btracing::error!("{}", e.translate(& lang));
                                }
                            }
                        },
                        {tr.btn_submit}
                    }
                }
            }
        }
    }
}

#[component]
pub fn MobileSupport(lang: Language) -> Element {
    let mut popup: PopupService = use_context();
    let tr: SupportTranslate = translate(&lang);
    let mut req = use_signal(|| SupportSubmitRequest::default());

    rsx! {
        div { id: "support",
            BackgroundTriangle { color: "#1E1E1E" }
            div { class: "w-screen h-full px-[30px] flex flex-col items-center justify-center",
                div { class: "w-full h-full flex flex-col gap-[40px]",
                    MobileSectionHeader {
                        section_name: tr.title,
                        title: tr.mission,
                        description: tr.description,
                        with_line: false,
                    }

                    div { class: "col-span-1 w-full flex flex-col items-center",
                        div { class: "col-span-1 w-full flex flex-col items-center gap-[30px]",
                            MobileLabeledInput {
                                class: "w-full",
                                label_name: tr.label_first_name,
                                placeholder: tr.placeholder_first_name,
                                oninput: move |e| {
                                    req.with_mut(move |r| r.first_name = e);
                                },
                            }
                            MobileLabeledInput {
                                class: "w-full",
                                label_name: tr.label_last_name,
                                placeholder: tr.placeholder_last_name,
                                oninput: move |e| {
                                    req.with_mut(move |r| r.last_name = e);
                                },
                            }

                            MobileLabeledInput {
                                class: "w-full",
                                label_name: tr.label_email,
                                placeholder: tr.placeholder_email,
                                oninput: move |e| {
                                    req.with_mut(move |r| r.email = e);
                                },
                            }

                            MobileLabeledInput {
                                class: "w-full",
                                label_name: tr.label_company,
                                placeholder: tr.placeholder_company,
                                oninput: move |e| {
                                    req.with_mut(move |r| r.company_name = e);
                                },
                            }

                            MobileLabeled { class: "w-full", label_name: tr.label_needs,
                                MobileDropdown {
                                    class: "w-full min-w-[333px] h-full min-h-[44px]",
                                    color: "ring-[#ffffff]",
                                    items: Need::variants(&lang),
                                    onselect: move |value: String| {
                                        req.with_mut(move |r| r.needs = value.parse().unwrap_or_default());
                                    },
                                }
                            }

                            MobileLabeledInput {
                                class: "w-full",
                                label_name: tr.label_help,
                                placeholder: tr.placeholder_help,
                                oninput: move |e| {
                                    req.with_mut(move |r| r.help = e);
                                },
                            }
                        } // end of form

                        SecondaryButton {
                            onclick: move |_| async move {
                                let endpoint = config::get().main_api_endpoint;
                                let SupportSubmitRequest {
                                    first_name,
                                    last_name,
                                    email,
                                    company_name,
                                    needs,
                                    help,
                                } = req();
                                match dto::Support::get_client(endpoint)
                                    .submit(first_name, last_name, email, company_name, needs, help)
                                    .await
                                {
                                    Ok(_) => {
                                        btracing::info!("Thank you for your submission!");
                                        let tr: InquiryTranslate = translate(&lang);
                                        popup.open(rsx! {
                                            ConfirmPopup {
                                                lang,
                                                title: tr.title,
                                                description: tr.description,
                                                btn_label: tr.btn_label,
                                            }
                                        });
                                    }
                                    Err(e) => {
                                        btracing::error!("{}", e.translate(& lang));
                                    }
                                }
                            },
                            div { class: "w-full min-w-[333px] h-[48px] flex justify-center items-center rounded-[10px] whitespace-nowrap bg-[#ffffff] p-[20px] font-bold text-[15px] text-black",
                                {tr.btn_submit}
                            }
                        }
                    }
                }
            }
        }
    }
}

translate! {
    SupportTranslate;

    title: {
        ko: "Support",
        en: "Support",
    },

    mission: {
        ko: "지원이 필요하신가요?",
        en: "Need Support?",
    },

    description: {
        ko: "문의하기 위해 양식을 작성하세요.",
        en: "Fill in the form to get in touch.",
    }

    label_first_name: {
        ko: "이름",
        en: "First Name",
    }

    placeholder_first_name: {
        ko: "이름을 입력하세요",
        en: "First name",
    }

    label_last_name: {
        ko: "성",
        en: "Last Name",
    }

    placeholder_last_name: {
        ko: "성을 입력하세요",
        en: "Last name",
    }

    label_email: {
        ko: "이메일",
        en: "Email",
    }

    placeholder_email: {
        ko: "이메일을 입력하세요",
        en: "Your email",
    }

    label_company: {
        ko: "회사",
        en: "Company name",
    }

    placeholder_company: {
        ko: "회사명을 입력하세요",
        en: "Company",
    }

    label_needs: {
        ko: "어떤 주제가 필요하신가요?",
        en: "Which topic best fit your needs?",
    }

    label_help: {
        ko: "어떤 도움이 필요하신가요?",
        en: "How can we help? ",
    }

    placeholder_help: {
        ko: "도움이 필요한 내용을 공유해주세요.",
        en: "Please share what you want us to help.",
    }

    btn_submit: {
        ko: "제출하기",
        en: "SUBMIT",
    }
}

translate! {
    InquiryTranslate;

    title: {
        ko: "문의가 접수되었습니다.",
        en: "Inquiry Received",
    },

    description: {
        ko: "문의해 주셔서 감사합니다. 귀하의 문의를 접수하였으며, 곧 회신드리겠습니다.",
        en: "Thank you for your inquiry. We have received your question and will respond to you shortly.",
    }

    btn_label: {
        ko: "확인",
        en: "Confirm",
    }
}
