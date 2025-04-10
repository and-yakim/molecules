use init::*;
use molecules::binned_arr::*;
use molecules::molecule::*;
use molecules::*;

const SIDE: f32 = 800.0;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let cell = Atom::<20>::RC;
    let side_n = (SIDE / cell) as usize;

    let mut gas = Atom::<20>::generate(SIDE, Vec2::splat(cell), 1.0);
    let mut binarr = BinnedArr::<usize>::new(side_n, cell, gas.len());
    println!("N: {}", gas.len());
    let camera = binarr.get_camera();

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        binarr.clear();
        for (i, mol) in gas.iter().enumerate() {
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
