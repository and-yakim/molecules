use molecules::gas::*;
use molecules::*;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let mut system = System::<20>::new(1000);

    println!("N: {}", system.matter.len());
    let camera = system.container.get_camera();

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        system.refresh_container();
        system.force_gas();
        system.move_gas();
        system.fix_bounds();

        system.draw();
        set_default_camera();
        draw_fps();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
