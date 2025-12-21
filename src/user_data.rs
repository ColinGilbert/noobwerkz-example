// This is where user-defined data structures live

use std::sync::{LazyLock, Mutex};

pub struct UserData {

}

impl UserData {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&mut self) -> bool {
        true
    }
}

pub static USER_DATA: LazyLock<Mutex<UserData>> = LazyLock::new(|| { Mutex::new(UserData::new()) } );