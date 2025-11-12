use raylib::prelude::*;

fn main() {
    // | grep -v '^INFO'
    let (mut rl, thread) = raylib::init().size(800, 800).title("Molecules").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
    }
}
