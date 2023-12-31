//! Provides a macro for quickly declaring numeric variables that can be tweaked through egui.
#![warn(missing_docs)]

// Token pasting macro used by the tweak macro internally
#[doc(hidden)]
pub use paste::paste;

/// Conveniently declare some numeric variables that can be tweaked through egui.
///
/// The types must statisfy the egui `Numeric` trait, and must be `Copy`.
///
/// Example usage:
///
/// ```
///     tweak! {
///        egui_ctx, // The egui context
///        health_bar, // Unique identifier for this group of tweakable variables
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
        $group_ident:ident,
        $($ident:ident: $t:ty = $init:expr;)*
    ) => {
        $crate::paste! {
            struct [<TweakGroup $group_ident:camel>] {
                $(
                    $ident: $t,
                )*
            }
            static [<TWEAK_ $group_ident:upper>]: ::std::sync::Mutex<[<TweakGroup $group_ident:camel>]> = std::sync::Mutex::new([<TweakGroup $group_ident:camel>] {
                $(
                    $ident: $init,
                )*
            });
            let mut tweak_local_guard = [<TWEAK_ $group_ident:upper>].lock().unwrap();
            egui::Window::new(stringify!($group_ident)).show($egui_ctx, |ui| {
                $(
                    ui.horizontal(|ui| {
                        ui.label(stringify!($ident));
                        ui.add(egui::DragValue::new(&mut tweak_local_guard.$ident));
                    });
                )*
            });
            $(
                    let $ident: $t = tweak_local_guard.$ident;
            )*
        }
    };
}
