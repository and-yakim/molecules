use init::*;
use molecules::gas::*;
use molecules::*;

type Particle = Atom<4>; // max 4
const CELL: Fixed = Particle::RC;
const SIZE: Fixed = fmul(CELL, 20); // 1k max (~1023 with 2 * CELL)
const _ACTUAL_SIZE: i32 = SIZE.to_bits() >> 21;

const fn corner_coords_default(coords: [usize; 2]) -> [[usize; 2]; 4] {
    let next_i = coords[0] + 1;
    let next_j = coords[1] + 1;
    [
        [coords[0], next_j],
        [next_i, next_j],
        [next_i, coords[1]],
        [next_i, coords[1] - 1],
    ]
}

const fn corner_coords(coords: [usize; 2], side: usize) -> [[usize; 2]; 4] {
    let next_i = (coords[0] + 1) % side;
    let next_j = (coords[1] + 1) % side;
    let prev_j = (coords[1] + side - 1) % side;
    [
        [coords[0], next_j],
        [next_i, next_j],
        [next_i, coords[1]],
        [next_i, prev_j],
    ]
}

const OFFSET_TOP: FVec2 = FVec2::new(Fixed::ZERO, fmul(SIZE, -1));
const OFFSET_BOTTOM: FVec2 = FVec2::new(Fixed::ZERO, SIZE);
const OFFSET_RIGHT: FVec2 = FVec2::new(SIZE, Fixed::ZERO);
const OFFSET_TOP_RIGHT: FVec2 = FVec2::new(SIZE, fmul(SIZE, -1));
const OFFSET_BOTTOM_RIGHT: FVec2 = FVec2::new(SIZE, SIZE);

#[derive(Clone, Copy)]
enum Offset {
    Top,
    Bottom,
    Right,
    TopRight,
    BottomRight,
}

impl Offset {
    fn to_fvec2(self) -> FVec2 {
        match self {
            Offset::Top => OFFSET_TOP,
            Offset::Bottom => OFFSET_BOTTOM,
            Offset::Right => OFFSET_RIGHT,
            Offset::TopRight => OFFSET_TOP_RIGHT,
            Offset::BottomRight => OFFSET_BOTTOM_RIGHT,
        }
    }
}

type IndexPair = (usize, usize, Option<Offset>);

fn refresh_sys(matter: &Vec<Particle>, system: &mut BinnedArr<usize>) {
    system.clear();
    for (i, mol) in matter.iter().enumerate() {
        system.add(mol.pos, i);
    }
}

fn get_chunk_iter(
    system: &BinnedArr<usize>,
    coords: [usize; 2],
) -> impl Iterator<Item = IndexPair> {
    system.arr[coords]
        .iter()
        .enumerate()
        .flat_map(move |(i, index)| {
            ((i + 1)..system.arr[coords].len()).map(move |j| {
                let other = system.arr[coords][j];
                (*index, other, None)
            })
        })
}

fn get_chunks_iter(
    system: &BinnedArr<usize>,
    x: [usize; 2],
    y: [usize; 2],
    option: Option<Offset>,
) -> impl Iterator<Item = IndexPair> {
    system.arr[x].iter().flat_map(move |index| {
        system.arr[y]
            .iter()
            .map(move |other| (*index, *other, option))
    })
}

fn force_pair(matter: &mut Vec<Particle>, (x, y, option): IndexPair) {
    let result = match option {
        // Some(offset) => matter[x].get_force_2(&matter[y], offset.to_fvec2()),
        // None => matter[x].get_force(&matter[y]),
        Some(offset) => Some(fvec2(0.0, 0.0)),
        None => Some(fvec2(0.0, 0.0)),
    };
    if let Some(force) = result {
        matter[x].vel += force;
        matter[y].vel -= force;
    }
}

fn force_gas(matter: &mut Vec<Particle>, system: &BinnedArr<usize>) {
    for i in 0..(system.side - 1) {
        for j in 1..(system.side - 1) {
            get_chunk_iter(system, [i, j])
                .chain(
                    corner_coords_default([i, j])
                        .iter()
                        .flat_map(|coords| get_chunks_iter(system, [i, j], *coords, None)),
                )
                .for_each(|pair| force_pair(matter, pair));
        }
    }
    for i in 0..(system.side - 1) {
        let j = 0;
        let corner = corner_coords([i, j], system.side);
        get_chunk_iter(system, [i, j])
            .chain(
                corner[..3]
                    .iter()
                    .flat_map(|coords| get_chunks_iter(system, [i, j], *coords, None)),
            )
            .chain(get_chunks_iter(
                system,
                [i, j],
                corner[3],
                Some(Offset::Top),
            ))
            .for_each(|pair| force_pair(matter, pair));
        let j = system.side - 1;
        let corner = corner_coords([i, j], system.side);
        get_chunk_iter(system, [i, j])
            .chain(
                corner[2..4]
                    .iter()
                    .flat_map(|coords| get_chunks_iter(system, [i, j], *coords, None)),
            )
            .chain(
                corner[..2].iter().flat_map(|coords| {
                    get_chunks_iter(system, [i, j], *coords, Some(Offset::Bottom))
                }),
            )
            .for_each(|pair| force_pair(matter, pair));
    }
    let [i, j] = [system.side - 1, 0];
    let corner = corner_coords([i, j], system.side);
    get_chunk_iter(system, [i, j])
        .chain(get_chunks_iter(system, [i, j], corner[0], None))
        .chain(
            corner[1..3]
                .iter()
                .flat_map(|coords| get_chunks_iter(system, [i, j], *coords, Some(Offset::Right))),
        )
        .chain(get_chunks_iter(
            system,
            [i, j],
            corner[3],
            Some(Offset::TopRight),
        ))
        .for_each(|pair| force_pair(matter, pair));
    let j = i;
    let corner = corner_coords([i, j], system.side);
    get_chunk_iter(system, [i, j])
        .chain(get_chunks_iter(
            system,
            [i, j],
            corner[0],
            Some(Offset::Bottom),
        ))
        .chain(get_chunks_iter(
            system,
            [i, j],
            corner[1],
            Some(Offset::BottomRight),
        ))
        .chain(
            corner[..2]
                .iter()
                .flat_map(|coords| get_chunks_iter(system, [i, j], *coords, Some(Offset::Right))),
        )
        .for_each(|pair| force_pair(matter, pair));
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
