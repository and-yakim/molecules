#![allow(unused)]
use molecules::init::*;
use molecules::soft_body::*;
use molecules::*;

#[macroquad::main("Rect body")]
async fn main() {
    init();
    let camera = get_camera(Vec2::ZERO, 1.0);

    let mut body = RectBody::new((20, 10), 30.0);
    let outer_iter = body.get_outer_indexes();

    let gravity = vec2(0.0, 0.01);
    let earth_y = SCREEN_SIDE / 2.0;

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        if is_key_down(KeyCode::Space) {
            let pos = default_world_pos(mouse_position().into());
            body.arr[[0, 0]].apply_spring_force(pos, 0.01, 0.0);
            body.arr[[0, 0]].draw_link_pos(pos);
        }
        if is_key_pressed(KeyCode::F) {
            outer_iter.iter().for_each(|i| {
                body.arr[*i].vel = Vec2::ZERO;
            });
        }

        body.update_with_ext_force(gravity);
        outer_iter.iter().for_each(|i| {
            if body.arr[*i].y() > earth_y {
                body.arr[*i].pos.y = earth_y;
                body.arr[*i].vel.y = -body.arr[*i].vel.y;
            } else {
                body.arr[*i].add_force_fn(|p| -0.01 * p.vel);
            }
        });
        // body.draw_outer();
        // body.draw_points();
        body.draw_full();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
