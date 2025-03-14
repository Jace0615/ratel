#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn PPTXFile(#[props(default = "white".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "36",
            height: "36",
            view_box: "0 0 36 36",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M30.8248 7.458V32.4C30.8248 34.0154 29.5153 35.325 27.8998 35.325H8.1C6.48457 35.325 5.175 34.0154 5.175 32.4V3.6C5.175 1.98457 6.48457 0.675 8.1 0.675H23.4401L30.8248 7.458Z",
                stroke: "#D0D5DD",
                stroke_width: "1.35",
            }
            mask { id: "path-2-inside-1_2019_7173", fill: "white",
                path { d: "M22.5 0.900391H30.15V8.55039H23.4C22.9029 8.55039 22.5 8.14745 22.5 7.65039V0.900391Z" }
            }
            path {
                d: "M22.5 0.900391H30.15H22.5ZM30.15 9.45039H23.4C22.4059 9.45039 21.6 8.6445 21.6 7.65039H23.4H30.15V9.45039ZM23.4 9.45039C22.4059 9.45039 21.6 8.6445 21.6 7.65039V0.900391H23.4V7.65039V9.45039ZM30.15 0.900391V8.55039V0.900391Z",
                fill: "#D0D5DD",
                mask: "url(#path-2-inside-1_2019_7173)",
            }
            rect {
                x: "0.898438",
                y: "18",
                width: "26.6",
                height: "11.4",
                rx: "1.8",
                fill: "#F97066",
            }
            path {
                d: "M8.08052 22.4162C8.08052 22.7822 7.99652 23.1182 7.82852 23.4242C7.66052 23.7242 7.40252 23.9672 7.05452 24.1532C6.70652 24.3392 6.27452 24.4322 5.75852 24.4322H4.80452V26.7002H3.26552V20.3822H5.75852C6.26252 20.3822 6.68852 20.4692 7.03652 20.6432C7.38452 20.8172 7.64552 21.0572 7.81952 21.3632C7.99352 21.6692 8.08052 22.0202 8.08052 22.4162ZM5.64152 23.2082C5.93552 23.2082 6.15452 23.1392 6.29852 23.0012C6.44252 22.8632 6.51452 22.6682 6.51452 22.4162C6.51452 22.1642 6.44252 21.9692 6.29852 21.8312C6.15452 21.6932 5.93552 21.6242 5.64152 21.6242H4.80452V23.2082H5.64152ZM13.6967 22.4162C13.6967 22.7822 13.6127 23.1182 13.4447 23.4242C13.2767 23.7242 13.0187 23.9672 12.6707 24.1532C12.3227 24.3392 11.8907 24.4322 11.3747 24.4322H10.4207V26.7002H8.88173V20.3822H11.3747C11.8787 20.3822 12.3047 20.4692 12.6527 20.6432C13.0007 20.8172 13.2617 21.0572 13.4357 21.3632C13.6097 21.6692 13.6967 22.0202 13.6967 22.4162ZM11.2577 23.2082C11.5517 23.2082 11.7707 23.1392 11.9147 23.0012C12.0587 22.8632 12.1307 22.6682 12.1307 22.4162C12.1307 22.1642 12.0587 21.9692 11.9147 21.8312C11.7707 21.6932 11.5517 21.6242 11.2577 21.6242H10.4207V23.2082H11.2577ZM19.0429 20.3822V21.6152H17.3689V26.7002H15.8299V21.6152H14.1559V20.3822H19.0429ZM23.6853 26.7002L22.3983 24.7652L21.2643 26.7002H19.5183L21.5433 23.4872L19.4733 20.3822H21.2643L22.5333 22.2902L23.6493 20.3822H25.3953L23.3883 23.5682L25.4763 26.7002H23.6853Z",
                fill: "white",
            }
        }
    }
}
