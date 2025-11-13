use molecules::gas::*;
use molecules::*;

fn main() {
    // | grep -v '^INFO'
    let (mut rl, thread) = raylib::init().size(800, 800).title("Molecules").build();

    let mut system = System::<20>::new(1000);

    println!("N: {}", system.matter.len());

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);

        system.refresh_container();
        system.force_gas();
        system.move_gas();
        system.fix_bounds();

        system.draw();

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
    }
}
