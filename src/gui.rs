
use yakui::{button, row, widgets::List, CrossAxisAlignment};
pub fn user_gui() {
        row(|| {
        button("Not stretched");
        let mut col = List::column();
        col.cross_axis_alignment = CrossAxisAlignment::Stretch;
        col.show(|| {
            button("Button 1");
            button("Button 2");
            button("Button 3");
        });
    });
}