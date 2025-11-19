use noobwerkz::{egui_renderer::EguiRenderer, user_context::UserContext};

#[allow(unused)]
pub fn user_gui(egui_renderer: &mut EguiRenderer, user_ctx: &mut UserContext) {
    noobwerkz::egui::Window::new("Noobwerkz engine!")
        .resizable(true)
        .min_height(80.0)
        .max_height(80.0)
        .vscroll(true)
        .default_open(true)
        .collapsible(false)
        .movable(false)
        .show(egui_renderer.context(), |ui| {
            ui.label("This is a demonstration of the game engine's capabilities. It'll get better over time.");
        });
}
