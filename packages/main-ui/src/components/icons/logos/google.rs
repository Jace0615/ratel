#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Google(
    #[props(default = "20".to_string())] width: String,
    #[props(default = "20".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            view_box: "0 0 20 20",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g { clip_path: "url(#clip0_264_8516)",
                path {
                    d: "M19.7614 10.1871C19.7614 9.36767 19.6949 8.76973 19.551 8.14966H10.1943V11.848H15.6865C15.5758 12.7671 14.9778 14.1512 13.6491 15.0813L13.6304 15.2051L16.5888 17.4969L16.7938 17.5174C18.6762 15.7789 19.7614 13.221 19.7614 10.1871Z",
                    fill: "#4285F4",
                }
                path {
                    d: "M10.1947 19.9313C12.8854 19.9313 15.1442 19.0454 16.7941 17.5174L13.6494 15.0813C12.8079 15.6682 11.6784 16.0779 10.1947 16.0779C7.55932 16.0779 5.3226 14.3395 4.52527 11.9366L4.4084 11.9466L1.33222 14.3273L1.29199 14.4391C2.93077 17.6945 6.29695 19.9313 10.1947 19.9313Z",
                    fill: "#34A853",
                }
                path {
                    d: "M3.14813 0.108311L3.36044 0.00732422L9.84946 5.04676L9.86107 5.32189C9.44584 6.61372 9.19218 7.99778 9.19218 9.42812C9.19218 10.8583 9.44584 12.2425 9.88413 13.5344L3.14813 18.7478C1.741 15.9334 0.933594 12.7731 0.933594 9.42812C0.933594 6.08318 1.741 2.92272 3.14813 0.108311Z",
                    fill: "#FBBC05",
                }
                path {
                    d: "M10.1947 3.85336C12.066 3.85336 13.3283 4.66168 14.048 5.33718L16.8605 2.59107C15.1332 0.985496 12.8854 0 10.1947 0C6.29695 0 2.93077 2.23672 1.29199 5.49214L4.51421 7.99466C5.3226 5.59183 7.55932 3.85336 10.1947 3.85336Z",
                    fill: "#EB4335",
                }
            }
            defs {
                clipPath { id: "clip0_264_8516",
                    rect { width, height, fill: "white" }
                }
            }
        }
    }
}
