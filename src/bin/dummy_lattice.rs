use molecules::init::*;
use molecules::spring::*;
use molecules::*;

#[macroquad::main("Dummy lattice")]
async fn main() {
    let camera = init_with_camera();

    let mut body = DummyLattice::new((10, 10), 20.0);

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        // if is_mouse_button_down(MouseButton::Left) {
        //     let pos = world_pos(mouse_position());
        //     let force = body.arr[[0, 0]].get_force(pos, 20.0);
        //     body.arr[[0, 0]].add_force(force);
        // }

        body.update();
        body.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
