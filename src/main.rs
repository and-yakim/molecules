use molecules::molecule::*;
use std::time;

const BORDER: f32 = 40.0;

fn init() -> Camera2D {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    request_new_screen_size(SIDE + BORDER, SIDE + BORDER);
    Camera2D {
        target: Vec2::ZERO,
        zoom: 2.0 * Vec2::ONE / vec2(SIDE + BORDER, SIDE + BORDER),
        ..Default::default()
    }
}

#[macroquad::main("Molecules")]
async fn main() {
    let camera = init();

    let mut gas = vec![Molecule::new(Vec2::ZERO, Vec2::ZERO)];

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        gas[0].draw();

        draw_rectangle_lines(-HALF, -HALF, SIDE, SIDE, 2.0, GREEN);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
