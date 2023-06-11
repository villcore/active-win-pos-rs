use std::thread::sleep;
use std::time::Duration;
use active_win_pos_rs::{get_active_window};

fn main() {
    loop {
        match get_active_window() {
            Ok(active_window) => {
                println!("active window: {:#?}", active_window);
            }
            Err(()) => {
                println!("error occurred while getting the active window");
            }
        }

        sleep(Duration::from_secs(1));
    }
}
