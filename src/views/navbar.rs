use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header {
            id: "navbar",
            class: "flex w-auto ml-10 mr-10 mt-5 bg-black rounded-4xl justify-between h-15",
            div {
                class: "ml-5 flex items-center",
                Link {
                    to: Route::Home {},
                    "Home"
                }
            }
            div {
                class: "mr-5 flex items-center justify-between gap-x-10",
                Link {
                    to: Route::GhRepos { },
                    "Github Repos" // TODO: responsive design
                }
                Link {
                    to: Route::GhRepos { },
                    "Github Repos" // TODO: responsive design
                }
            }
        }

        Outlet::<Route> {}
    }
}
