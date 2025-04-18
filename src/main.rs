use init::*;
use molecules::gas::*;
use molecules::*;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let mut gas = Gas::new(1.0);
    println!("N: {}", gas.value.len());
    let camera = gas.system.get_camera();

    gas.refresh_sys();

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        gas.force_gas();
        // gas.move_gas();

        gas.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
