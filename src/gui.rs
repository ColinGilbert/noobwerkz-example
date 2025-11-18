use noobwerkz::{egui_tools::EguiRenderer, user_context::UserContext};


pub fn user_gui(egui_renderer: &mut EguiRenderer, user_ctx: &mut UserContext) {
        noobwerkz::egui::Window::new("Noobwerkz engine!")
            .resizable(true)
            .vscroll(true)
            .default_open(false)
            .movable(false)
            .show(egui_renderer.context(), |ui| {
                ui.label("Label!");

                if ui.button("Button!").clicked() {
                    println!("boom!")
                }

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Pixels per point: {}",
                        egui_renderer.context().pixels_per_point()
                    ));
                    if ui.button("-").clicked() {
                        // state.scale_factor = (state.scale_factor - 0.1).max(0.3);
                    }
                    if ui.button("+").clicked() {
                        // state.scale_factor = (state.scale_factor + 0.1).min(3.0);
                    }
                });
            });
}