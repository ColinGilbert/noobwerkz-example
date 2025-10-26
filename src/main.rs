use noobwerkz::app::run;
use noobwerkz::callbacks::*;
use noobwerkz_example::setup::*;
use noobwerkz_example::update::*;

fn main() {
    init_user_setup_callback(user_setup_implementation);
    init_user_update_callback(user_update_implementation);
    run().unwrap();
}
