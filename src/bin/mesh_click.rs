#![allow(unused)]
use molecules::init::*;
use molecules::mesh::*;
use molecules::*;

#[macroquad::main("Mesh click")]
async fn main() {
    init();
    let camera = get_camera(Vec2::ZERO, 1.0);

    let mesh = SpringMesh::new(Vec2::ZERO, 30.0);

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        let pos = round_to_triangular_grid(default_world_pos(mouse_position().into()), mesh.cell);
        draw_circle(pos.x, pos.y, 4.0, DEBUG_RED);
        mesh.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
