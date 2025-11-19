use noobwerkz::app::run;
use noobwerkz::callbacks::*;
use noobwerkz_example::setup::*;
use noobwerkz_example::update::*;
use noobwerkz_example::gui::*;
use noobwerkz_example::user_struct::*;
fn main() {

    init_user_setup_callback(user_setup);
    init_user_update_callback(user_update);
    init_user_gui_callback(user_gui);
    run().unwrap();
}
