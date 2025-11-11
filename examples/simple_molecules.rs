use molecules::gas::*;
use molecules::*;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let mut system = System::<20>::new(1000);

    println!("N: {}", system.matter.len());
    let camera = system.container.get_camera();

    loop {
        clear_background(LIGHTGRAY);
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
        if is_key_pressed(KeyCode::Down) {
            for mol in &mut system.matter {
                mol.vel *= Fixed::from_bits(1 << (FRAC_BITS - 1));
            }
        } else if is_key_pressed(KeyCode::Up) {
            for mol in &mut system.matter {
                mol.vel *= Fixed::from_bits(2 << FRAC_BITS);
            }
        }
        next_frame().await;
    }
}
