use init::*;
use molecules::binned_arr::*;
use molecules::molecule::*;
use molecules::*;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let cell = 100.0;
    let radius = 20.0;
    let side = (SIDE / (radius * 2.0)) as usize;
    let n = side * side;

    let mut binarr = BinnedArr::<usize>::new(side, cell, n);
    let camera = binarr.get_camera();

    let gas = Atom::generate(cell);
    println!("LEN: {}", gas.len());

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
