#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn RatelCircle(
    #[props(default = "white".to_string())] color: String,
    #[props(default = "100".to_string())] size: String,
) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 194 197",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g { clip_path: "url(#clip0_811_3538)",
                path {
                    d: "M165.951 59.0237C164.032 55.6614 161.881 52.4505 159.52 49.4119C175.539 29.669 189.182 11.3302 193.75 2.28653C185.086 7.06915 167.094 21.0887 147.882 37.2714C145.151 34.9569 142.266 32.8194 139.241 30.8775C149.563 17.7614 157.848 6.25653 161.007 0C154.544 3.56718 141.951 13.2069 127.923 24.8189C118.001 20.4856 107.041 18.0804 95.5223 18.0804C50.7743 18.0804 14.4995 54.3552 14.4995 99.1032C14.4995 111.36 17.2214 122.981 22.0925 133.397C11.9847 146.145 3.80492 157.324 0.75 163.289C6.18226 160.332 16.2015 152.797 27.7087 143.465C29.9347 146.86 32.4075 150.075 35.1015 153.09C20.77 170.836 8.70163 186.979 4.57098 195.042C12.5063 190.723 28.7262 178.135 46.313 163.482C49.5774 165.983 53.0399 168.239 56.6699 170.226C47.7613 181.553 40.6432 191.37 37.8887 196.751C43.6166 193.633 54.6721 185.197 67.092 175.001C75.9378 178.317 85.517 180.131 95.52 180.131C140.268 180.131 176.543 143.856 176.543 99.1079C176.543 88.6229 174.552 78.6013 170.924 69.4039C180.578 57.1051 188.301 46.3523 191.277 40.4614C186.082 43.3277 176.755 50.2921 165.948 59.0284L165.951 59.0237ZM95.5223 156.332C63.9183 156.332 38.2961 130.712 38.2961 99.1055C38.2961 67.4993 63.916 41.8794 95.5223 41.8794C127.129 41.8794 152.748 67.4993 152.748 99.1055C152.748 130.712 127.129 156.332 95.5223 156.332Z",
                    fill: "{color}",
                }
            }
            defs {
                clipPath { id: "clip0_811_3538",
                    rect {
                        width: "193",
                        height: "196.749",
                        fill: "white",
                        transform: "translate(0.75)",
                    }
                }
            }
        }
    }
}
