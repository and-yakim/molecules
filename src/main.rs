use init::*;
use molecules::gas::*;
use molecules::*;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let mut gas = Gas::new(1.0);
    println!("N: {}", gas.value.len());
    let camera = gas.system.get_camera();

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        gas.refresh_sys();

        // gas.force_gas();
        gas.move_gas();

        gas.draw();

        set_default_camera();
        draw_fps();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
