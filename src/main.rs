use init::*;
use molecules::binned_arr::*;
use molecules::molecule::*;
use molecules::*;

const SIZE: f32 = 800.0;
type Particle = Atom<20>;

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let cell = Particle::RC;
    let far_border = cell + SIZE;

    let mut gas = Particle::generate(SIZE, Vec2::splat(cell), 10.0);
    let mut binarr = BinnedArr::<usize>::new(SIZE, cell, gas.len());
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

        for mol in gas.iter_mut() {
            mol.move_pos();
        }

        for n in 0..binarr.side {
            binarr.arr[[n, 0]].iter().for_each(|i| {
                if gas[*i].pos.x < binarr.cell {
                    gas[*i].pos.x += SIZE;
                }
            });
            binarr.arr[[n, binarr.side - 1]].iter().for_each(|i| {
                if gas[*i].pos.x > far_border {
                    gas[*i].pos.x -= SIZE;
                }
            });
            binarr.arr[[0, n]].iter().for_each(|i| {
                if gas[*i].pos.y < binarr.cell {
                    gas[*i].pos.y += SIZE;
                }
            });
            binarr.arr[[binarr.side - 1, n]].iter().for_each(|i| {
                if gas[*i].pos.y > far_border {
                    gas[*i].pos.y = SIZE;
                }
            });
        }

        for mol in gas.iter_mut() {
            mol.draw();
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
