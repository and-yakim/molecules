use init::*;
use molecules::binned_arr::*;
use molecules::molecule::*;
use molecules::*;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let cell = 100.0;
    let side_n: usize = (SIDE / cell) as usize;

    let gas = Atom::<20>::generate(cell);
    let mut binarr = BinnedArr::<usize>::new(side_n, cell, gas.len());
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
