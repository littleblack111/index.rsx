use crate::Route;
use crate::components::RoutesOutlet;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header {
            id: "navbar",
            class: "flex w-auto ml-10 mr-10 mt-5 bg-black rounded-4xl justify-between h-15 transition-all duration-500 hover:scale-105 active:scale-101",
            div {
                class: "ml-5 flex items-center",
                RouteLink { to: Route::Home { }, name: "Home" }
            }
            div {
                class: "mr-5 flex items-center justify-between gap-x-10",
                RouteLink { to: Route::GhRepos { }, name: "Github Repos" }
            }
        }

        RoutesOutlet {}
    }
}

#[component]
pub fn RouteLink(to: Route, name: &'static str) -> Element {
    let route = use_route::<Route>();
    let base_underline = "relative after:absolute after:left-0 after:bottom-0 after:w-full after:h-[.5mm] hover:after:h-[.8mm] after:rounded-[1mm] after:scale-x-100 hover:after:scale-x-105 after:transition-all";
    let underline = if route == to {
        format!("{base_underline} after:bg-[rgba(128,128,128,0.9)]")
    } else {
        format!("{base_underline} hover:after:bg-[rgba(128,128,128,0.5)]")
    };

    // this gives the path
    // let name: String = to
    //     .to_string()
    //     .chars()
    //     .skip(1)
    //     .collect();

    rsx! {
        Link {
            class: "text-theme-accent text-xl transition-all hover:scale-110 hover:font-bold {underline}",
            to,
            "{name}"
        }
    }
}
