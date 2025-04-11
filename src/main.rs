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

    let mut gas = Particle::generate(SIDE, Vec2::splat(cell), 2.0);
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

        (0..binarr.side).for_each(|i| {
            (0..binarr.side).for_each(|j| {
                binarr.update_by_fn([i, j], |x, y| {
                    if let Some(force) = gas[*x].get_force(&gas[*y]) {
                        gas[*x].vel += force;
                        gas[*y].vel -= force;
                    }
                });
            })
        });

        // (0..binarr.side).for_each(|i| {
        //     binarr.arr[i, 0]
        // });

        for mol in gas.iter_mut() {
            mol.move_pos();
            mol.draw();
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
