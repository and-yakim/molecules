use molecules::soft_body::*;
use molecules::*;

#[macroquad::main("Soft wheel")]
async fn main() {
    init();
    let camera = get_camera(Vec2::ZERO, 1.0);

    let mut wheel = Wheel::new(Vec2::ZERO, 100.0, 20);

    let gravity = vec2(0.0, 0.01 * wheel.len as f32);
    let earth_y = SCREEN_SIDE / 2.0;

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        if is_key_down(KeyCode::Space) {
            let pos = default_world_pos(mouse_position().into());
            wheel.center_mut().apply_spring_force(pos, 0.01, 0.0);
            wheel.center().draw_link_pos(pos);
        }
        if is_key_pressed(KeyCode::F) {
            wheel.iter_outer_indexes().for_each(|i| {
                wheel.arr[i].vel = Vec2::ZERO;
            });
        }
        if is_key_down(KeyCode::G) {
            wheel.center_mut().add_force(gravity * 20.0);
        }

        wheel.update_with_ext_force(gravity);
        wheel.iter_outer_indexes().for_each(|i| {
            if wheel.arr[i].y() > earth_y {
                wheel.arr[i].pos.y = earth_y;
                wheel.arr[i].vel.y = -wheel.arr[i].vel.y;
            } else {
                wheel.arr[i].add_force_fn(|p| -0.01 * p.vel);
            }
        });
        wheel.draw_full();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
