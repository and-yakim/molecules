use molecules::init::*;
use molecules::spring::*;
use molecules::*;

#[macroquad::main("Soft body")]
async fn main() {
    let camera = init_with_camera();

    let body = SoftBody::new(10, 10);

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        body.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
