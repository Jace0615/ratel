#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn OutlinedVoteYes() -> Element {
    rsx! {
        svg {
            width: "17",
            height: "17",
            view_box: "0 0 17 17",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M9.8333 6.49988H9.0833C9.0833 6.91409 9.41909 7.24988 9.8333 7.24988V6.49988ZM7.8333 1.83321V1.08321C7.53691 1.08321 7.26832 1.25776 7.14795 1.52861L7.8333 1.83321ZM5.16664 7.83321L4.48128 7.52861C4.43866 7.6245 4.41664 7.72827 4.41664 7.83321H5.16664ZM5.16664 15.1665H4.41664C4.41664 15.5808 4.75242 15.9165 5.16664 15.9165V15.1665ZM12.6866 15.1665L12.6951 14.4165H12.6866V15.1665ZM14.02 14.0332L13.2786 13.9195L13.2784 13.9208L14.02 14.0332ZM14.94 8.03321L15.6813 8.14688L15.6815 8.14576L14.94 8.03321ZM13.6066 6.49988V7.24993L13.6151 7.24983L13.6066 6.49988ZM5.16664 15.1666V15.9166C5.58085 15.9166 5.91664 15.5808 5.91664 15.1666H5.16664ZM3.38664 15.1666V14.4165L3.37337 14.4167L3.38664 15.1666ZM1.8333 13.8333H1.0833C1.0833 13.8669 1.08556 13.9005 1.09007 13.9338L1.8333 13.8333ZM1.8333 9.16663L1.09007 9.06609C1.08556 9.09941 1.0833 9.133 1.0833 9.16663H1.8333ZM3.38664 7.83329L3.37337 8.58329H3.38664V7.83329ZM5.16664 7.83329H5.91664C5.91664 7.41908 5.58085 7.08329 5.16664 7.08329V7.83329ZM10.5833 6.49988V3.83321H9.0833V6.49988H10.5833ZM10.5833 3.83321C10.5833 2.31443 9.35209 1.08321 7.8333 1.08321V2.58321C8.52366 2.58321 9.0833 3.14286 9.0833 3.83321H10.5833ZM7.14795 1.52861L4.48128 7.52861L5.852 8.13782L8.51866 2.13782L7.14795 1.52861ZM4.41664 7.83321V15.1665H5.91664V7.83321H4.41664ZM5.16664 15.9165H12.6866V14.4165H5.16664V15.9165ZM12.6782 15.9165C13.717 15.9282 14.6057 15.1728 14.7615 14.1457L13.2784 13.9208C13.2348 14.2084 12.986 14.4199 12.6951 14.4166L12.6782 15.9165ZM14.7613 14.1469L15.6813 8.14688L14.1986 7.91954L13.2786 13.9195L14.7613 14.1469ZM15.6815 8.14576C15.7732 7.54144 15.5949 6.92733 15.1938 6.46609L14.0619 7.45036C14.1742 7.5795 14.2241 7.75146 14.1985 7.92067L15.6815 8.14576ZM15.1938 6.46609C14.7927 6.00484 14.2093 5.74301 13.5981 5.74993L13.6151 7.24983C13.7863 7.24789 13.9496 7.32121 14.0619 7.45036L15.1938 6.46609ZM13.6066 5.74988H9.8333V7.24988H13.6066V5.74988ZM5.16664 14.4166H3.38664V15.9166H5.16664V14.4166ZM3.37337 14.4167C2.97274 14.4238 2.63025 14.1298 2.57654 13.7328L1.09007 13.9338C1.24577 15.0849 2.23855 15.937 3.3999 15.9165L3.37337 14.4167ZM2.5833 13.8333V9.16663H1.0833V13.8333H2.5833ZM2.57654 9.26716C2.63025 8.87007 2.97274 8.57609 3.37337 8.58318L3.3999 7.08341C2.23855 7.06287 1.24577 7.91505 1.09007 9.06609L2.57654 9.26716ZM3.38664 8.58329H5.16664V7.08329H3.38664V8.58329ZM4.41664 7.83329V15.1666H5.91664V7.83329H4.41664Z",
                fill: "#68D36C",
            }
        }
    }
}

#[component]
pub fn FilledVoteYes(#[props(default = "#68D36C".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "17",
            height: "17",
            view_box: "0 0 17 17",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            // First Path
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M9.83344 6.49992V3.83325C9.83344 2.72868 8.93801 1.83325 7.83344 1.83325L6.05566 6.94436V15.1666H12.6868C13.3516 15.1741 13.9204 14.6906 14.0201 14.0333L14.9401 8.03325C14.9988 7.64649 14.8847 7.25346 14.628 6.95826C14.3713 6.66306 13.9979 6.49549 13.6068 6.49992H9.83344Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }

            // Second Path
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M3.38881 15.1666H3.16659C2.43021 15.1666 1.83325 14.5696 1.83325 13.8333V9.16659C1.83325 8.43021 2.43021 7.83325 3.16659 7.83325H3.38881V15.1666Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
        }
    }
}

#[component]
pub fn OutlinedVoteNo() -> Element {
    rsx! {
        svg {
            width: "17",
            height: "17",
            view_box: "0 0 17 17",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M9.8333 10.5001H9.0833C9.0833 10.0859 9.41909 9.75012 9.8333 9.75012V10.5001ZM7.8333 15.1668V15.9168C7.53691 15.9168 7.26832 15.7422 7.14795 15.4714L7.8333 15.1668ZM5.16664 9.16679L4.48128 9.47139C4.43866 9.3755 4.41664 9.27173 4.41664 9.16679H5.16664ZM5.16664 1.83345H4.41664C4.41664 1.41924 4.75242 1.08345 5.16664 1.08345V1.83345ZM12.6866 1.83345L12.6951 2.58345H12.6866V1.83345ZM14.02 2.96679L13.2786 3.08046L13.2784 3.07924L14.02 2.96679ZM14.94 8.96679L15.6813 8.85312L15.6815 8.85424L14.94 8.96679ZM13.6066 10.5001V9.75007L13.6151 9.75017L13.6066 10.5001ZM5.16664 1.83337V1.08337C5.58085 1.08337 5.91664 1.41916 5.91664 1.83337H5.16664ZM3.38664 1.83337V2.58349L3.37337 2.58326L3.38664 1.83337ZM1.8333 3.16671H1.0833C1.0833 3.13308 1.08556 3.0995 1.09007 3.06618L1.8333 3.16671ZM1.8333 7.83337L1.09007 7.93391C1.08556 7.90059 1.0833 7.867 1.0833 7.83337H1.8333ZM3.38664 9.16671L3.37337 8.41671H3.38664V9.16671ZM5.16664 9.16671H5.91664C5.91664 9.58092 5.58085 9.91671 5.16664 9.91671V9.16671ZM10.5833 10.5001V13.1668H9.0833V10.5001H10.5833ZM10.5833 13.1668C10.5833 14.6856 9.35209 15.9168 7.8333 15.9168V14.4168C8.52366 14.4168 9.0833 13.8571 9.0833 13.1668H10.5833ZM7.14795 15.4714L4.48128 9.47139L5.852 8.86218L8.51866 14.8622L7.14795 15.4714ZM4.41664 9.16679V1.83345H5.91664V9.16679H4.41664ZM5.16664 1.08345H12.6866V2.58345H5.16664V1.08345ZM12.6782 1.0835C13.717 1.07176 14.6057 1.82717 14.7615 2.85433L13.2784 3.07924C13.2348 2.79163 12.986 2.58012 12.6951 2.58341L12.6782 1.0835ZM14.7613 2.85312L15.6813 8.85312L14.1986 9.08046L13.2786 3.08046L14.7613 2.85312ZM15.6815 8.85424C15.7732 9.45856 15.5949 10.0727 15.1938 10.5339L14.0619 9.54964C14.1742 9.4205 14.2241 9.24854 14.1985 9.07933L15.6815 8.85424ZM15.1938 10.5339C14.7927 10.9952 14.2093 11.257 13.5981 11.2501L13.6151 9.75017C13.7863 9.75211 13.9496 9.67879 14.0619 9.54964L15.1938 10.5339ZM13.6066 11.2501H9.8333V9.75012H13.6066V11.2501ZM5.16664 2.58337H3.38664V1.08337H5.16664V2.58337ZM3.37337 2.58326C2.97274 2.57617 2.63025 2.87015 2.57654 3.26724L1.09007 3.06618C1.24577 1.91513 2.23855 1.06295 3.3999 1.08349L3.37337 2.58326ZM2.5833 3.16671V7.83337H1.0833V3.16671H2.5833ZM2.57654 7.73284C2.63025 8.12993 2.97274 8.42391 3.37337 8.41682L3.3999 9.91659C2.23855 9.93713 1.24577 9.08495 1.09007 7.93391L2.57654 7.73284ZM3.38664 8.41671H5.16664V9.91671H3.38664V8.41671ZM4.41664 9.16671V1.83337H5.91664V9.16671H4.41664Z",
                fill: "#FF5A5D",
            }
        }
    }
}

#[component]
pub fn FilledVoteNo(#[props(default="#FF5A5D".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "17",
            height: "17",
            view_box: "0 0 17 17",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            // First Path
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M9.83344 10.5001V13.1667C9.83344 14.2713 8.93801 15.1667 7.83344 15.1667L6.05566 10.0556V1.83342H12.6868C13.3516 1.8259 13.9204 2.30936 14.0201 2.96675L14.9401 8.96675C14.9988 9.35351 14.8847 9.74654 14.628 10.0417C14.3713 10.3369 13.9979 10.5045 13.6068 10.5001H9.83344Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }

            // Second Path
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M3.38881 1.83341H3.16659C2.43021 1.83341 1.83325 2.43037 1.83325 3.16675V7.83341C1.83325 8.56979 2.43021 9.16675 3.16659 9.16675H3.38881V1.83341Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
        }
    }
}
