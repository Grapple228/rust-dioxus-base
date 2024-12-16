use dioxus::prelude::*;
use lib_ui::Hero;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}
