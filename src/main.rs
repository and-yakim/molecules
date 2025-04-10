use init::*;
use molecules::binned_arr::*;
use molecules::molecule::*;
use molecules::*;

const SIDE: f32 = 1000.0;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let cell = Atom::<20>::RC;
    let side_n = (SIDE / cell) as usize;

    let gas = Atom::<20>::generate(SIDE);
    let mut binarr = BinnedArr::<usize>::new(side_n, cell, gas.len());

    println!("N: {}", gas.len());
    let camera = binarr.get_camera();

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        for mol in &gas {
            mol.draw()
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
