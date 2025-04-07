use molecules::molecule::*;
use std::time;

#[macroquad::main("Molecules")]
async fn main() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }

    loop {
        clear_background(DARKGRAY);

        next_frame().await;
    }
}
