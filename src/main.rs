use init::*;
use molecules::binned_arr::*;
use molecules::molecule::*;
use molecules::*;

const SIDE: f32 = 800.0;
type Particle = Atom<20>;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let cell = Particle::RC;
    let side_n = (SIDE / cell) as usize;

    let mut gas = Particle::generate(SIDE, Vec2::splat(cell), 1.0);
    let mut binarr = BinnedArr::<i16>::new(side_n, cell, gas.len());
    println!("N: {}", gas.len());
    let camera = binarr.get_camera();

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        binarr.clear();
        for (i, mol) in (0..gas.len() as i16).zip(gas.iter()) {
            binarr.add(mol.pos, i);
        }

        for mol in &gas {
            mol.draw()
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
