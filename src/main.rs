use init::*;
use molecules::molecule::*;
use molecules::*;

#[macroquad::main("Molecules")]
async fn main() {
    let camera = init_with_camera();

    let gas = Atom::generate();
    println!("LEN: {}", gas.len());

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        for mol in &gas {
            mol.draw()
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
