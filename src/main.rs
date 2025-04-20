use init::*;
use molecules::gas::*;
use molecules::*;

type Particle = Atom<4>; // max 4
const CELL: Fixed = Particle::RC;
const SIZE: Fixed = fmul(CELL, 20); // 1k max (~1023 with 2 * CELL)
const _ACTUAL_SIZE: i32 = SIZE.to_bits() >> 21;

const fn corner_coords(coords: [usize; 2]) -> [[usize; 2]; 4] {
    let next_i = coords[0] + 1;
    let next_j = coords[1] + 1;
    let prev_j = coords[1] - 1;
    [
        [coords[0], next_j],
        [next_i, next_j],
        [next_i, coords[1]],
        [next_i, prev_j],
    ]
}

const OFFSET_TOP: FVec2 = FVec2::new(Fixed::ZERO, fmul(SIZE, -1));
const OFFSET_BOTTOM: FVec2 = FVec2::new(Fixed::ZERO, SIZE);
const OFFSET_TOP_RIGHT: FVec2 = FVec2::new(SIZE, fmul(SIZE, -1));
const OFFSET_BOTTOM_RIGHT: FVec2 = FVec2::new(SIZE, SIZE);

fn refresh_sys(matter: &Vec<Particle>, system: &mut BinnedArr<usize>) {
    system.clear();
    for (i, mol) in matter.iter().enumerate() {
        system.add(mol.pos, i);
    }
}

fn update_chunk<F: FnMut(usize, usize)>(system: &BinnedArr<usize>, coords: [usize; 2], mut f: F) {
    for (i, index) in system.arr[coords].iter().enumerate() {
        for j in (i + 1)..system.arr[coords].len() {
            let other = system.arr[coords][j];
            f(*index, other);
        }
    }
}

fn update_chunks_by_fn<F>(system: &BinnedArr<usize>, x: [usize; 2], y: [usize; 2], mut f: F)
where
    F: FnMut(usize, usize),
{
    system.arr[x].iter().for_each(|index| {
        system.arr[y].iter().for_each(|other| f(*index, *other));
    });
}

fn force_particles(matter: &mut Vec<Particle>, x: usize, y: usize) {
    if let Some(force) = matter[x].get_force(&matter[y]) {
        matter[x].vel += force;
        matter[y].vel -= force;
    }
}

fn force_gas(matter: &mut Vec<Particle>, system: &BinnedArr<usize>) {
    for i in 0..(system.side - 1) {
        for j in 1..(system.side - 1) {
            for coords in corner_coords([i, j]) {
                // update_chunk(system, [i, j], |x, y| force_particles(matter, x, y));
                // update_chunks_by_fn(system, [i, j], coords, |x, y| force_particles(matter, x, y));
            }
        }
    }
    for i in 0..(system.side - 1) {
        let j = 0;
        // let corner = corner_coords([i, j]);
        // OFFSET_TOP
        let j = system.side - 1;
        // let corner = corner_coords([i, j]);
        // OFFSET_BOTTOM
    }
    let [i, j] = [system.side - 1, 0];
    // OFFSET_TOP_RIGHT
    let j = i;
    // OFFSET_BOTTOM_RIGHT
}

/// [CELL, CELL + SIZE)
fn wrap_range(num: Fixed) -> Fixed {
    ((num - CELL) % SIZE + SIZE) % SIZE + CELL
}
fn fix_bounds(matter: &mut Vec<Particle>, system: &BinnedArr<usize>) {
    for k in 0..system.side {
        system.arr[[0, k]]
            .iter()
            .chain(system.arr[[system.side - 1, k]].iter())
            .for_each(|i| {
                matter[*i].pos.x = wrap_range(matter[*i].pos.x);
            });
        system.arr[[k, 0]]
            .iter()
            .chain(system.arr[[k, system.side - 1]].iter())
            .for_each(|i| {
                matter[*i].pos.y = wrap_range(matter[*i].pos.y);
            });
    }
}

fn move_gas(matter: &mut Vec<Particle>, system: &BinnedArr<usize>) {
    for mol in matter.iter_mut() {
        mol.move_pos();
    }
    fix_bounds(matter, system);
}

fn draw(matter: &mut Vec<Particle>) {
    matter.iter().for_each(Atom::draw);
}

#[macroquad::main("Molecules")]
async fn main() {
    init();
    let mut matter = Particle::generate(SIZE, FVec2::new(CELL, CELL), 1.0);
    let mut system = BinnedArr::<usize>::new(SIZE, CELL, matter.len());

    println!("N: {}", matter.len());
    let camera = system.get_camera();

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        refresh_sys(&matter, &mut system);

        force_gas(&mut matter, &system);
        move_gas(&mut matter, &system);

        draw(&mut matter);

        set_default_camera();
        draw_fps();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
