use molecules::{gas::*, init_rand};

fn main() {
    init_rand();

    let mut system = System::<20>::new(1000);
    println!("N: {}", system.matter.len());

    // | grep -v '^INFO'
    let (mut rl, thread) = raylib::init().size(800, 800).title("Molecules").build();
    let camera = system.container.get_camera(800.0);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            for mol in &mut system.matter {
                mol.vel *= Fixed::from_bits(1 << (FRAC_BITS - 1));
            }
        } else if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            for mol in &mut system.matter {
                mol.vel *= Fixed::from_bits(2 << FRAC_BITS);
            }
        }

        system.refresh_container();
        system.force_gas();
        system.move_gas();
        system.fix_bounds();

        let mut d = rl.begin_drawing(&thread);
        let mut d = d.begin_mode2D(camera);

        d.clear_background(Color::GRAY);
        system.draw(&mut d);
    }
}
