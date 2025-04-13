use molecules::init::*;
use molecules::*;

#[macroquad::main("try")]
async fn main() {
    init();
    let camera = Camera3D {
        position: vec3(0.0, 100.0, 0.0),
        ..Default::default()
    };

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);
        draw_sphere(Vec3::ZERO, 4.0, None, RED);

        set_default_camera();
        draw_circle(100.0, 100.0, 4.0, GREEN);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
