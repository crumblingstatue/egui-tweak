//! Provides a macro for quickly declaring numeric variables that can be tweaked through egui.
#![warn(missing_docs)]

/// Token pasting macro used by the tweak macro internally
pub use paste::paste;

/// Conveniently declare some numeric variables that can be tweaked through egui.
///
/// The types must statisfy the egui `Numeric` trait, and must be `Clone`.
///
/// Example usage:
///
/// ```
///     tweak! {
///        egui_ctx,
///        "Health bar",
///        // We want to draw a health bar, but don't know where on the screen it should be.
///        // We want to experiment at runtime, so we just give some rough initial values,
///        // and tweak them with an egui ui until they are just right.
///        left: f32 = 0.;
///        top: f32 = 0.;
///        width: f32 = 100.;
///        height: f32 = 100.;
///     }
///     // The variables can then be used like normal locals
///     draw_health_bar(left, top, width, height);
/// ```
///
#[macro_export]
macro_rules! tweak {
    (
        $egui_ctx: expr,
        $window_name:expr,
        $($ident:ident: $t:ty = $init:expr;)*
    ) => {
        $crate::paste! {
            $(
                static [<TWEAK_ $ident:upper>]: Mutex<$t> = Mutex::new($init);
            )*
            egui::Window::new($window_name).show($egui_ctx, |ui| {
                $(
                    ui.horizontal(|ui| {
                        ui.label(stringify!($ident));
                        ui.add(egui::DragValue::new(&mut *[<TWEAK_ $ident:upper>].lock().unwrap()));
                    });
                )*
            });
            $(
                    let $ident: $t = *[<TWEAK_ $ident:upper>].lock().unwrap();
            )*
        }
    };
}
