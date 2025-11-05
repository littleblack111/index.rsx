use crate::Route;
use dioxus::prelude::*;
#[cfg(feature = "motion")]
use dioxus_motion::prelude::*;

#[cfg(feature = "motion")]
#[component]
pub fn RoutesOutlet() -> Element {
    rsx! { AnimatedOutlet::<Route> {} }
}

#[cfg(not(feature = "motion"))]
#[component]
pub fn RoutesOutlet() -> Element {
    rsx! { Outlet::<Route> {} }
}
