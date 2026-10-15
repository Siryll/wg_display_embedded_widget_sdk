#[doc(hidden)]
pub mod bindings {
    wit_bindgen::generate!({
        path: "wg_display_widget_wit/wit",
        world: "widget",
        pub_export_macro: true,
        default_bindings_module: "wg_display_widget::bindings",
    });
}

pub use bindings::widget::widget::{clocks, http, logging, random};
pub use bindings::{export, Guest, WidgetContext, WidgetResult};
