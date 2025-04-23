use init::*;
use molecules::gas::*;
use molecules::*;

type Particle = Atom<20>; // max 22
const CELL: Fixed = Particle::RC;
const SIZE: Fixed = fmul(CELL, 20); // ~32k max (with 2 * CELL)
const _ACTUAL_SIZE: i32 = SIZE.to_bits() >> FRAC_BITS;

fn get_corner_def(coords: [usize; 2]) -> [[usize; 2]; 4] {
    let next_i = coords[0] + 1;
    let next_j = coords[1] + 1;
    [
        [coords[0], next_j],
        [next_i, next_j],
        [next_i, coords[1]],
        [next_i, coords[1] - 1],
    ]
}

fn get_corner(coords: [usize; 2], side: usize) -> [[usize; 2]; 4] {
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

#[derive(Clone, Copy, Debug)]
enum Offset {
    Top,
    Bottom,
    Left,
    Right,
    TopRight,
    BottomRight,
}

impl Offset {
    const fn to_fvec2(self) -> FVec2 {
        match self {
            Offset::Top => FVec2::new(Fixed::ZERO, fmul(SIZE, -1)),
            Offset::Bottom => FVec2::new(Fixed::ZERO, SIZE),
            Offset::Left => FVec2::new(fmul(SIZE, -1), Fixed::ZERO),
            Offset::Right => FVec2::new(SIZE, Fixed::ZERO),
            Offset::TopRight => FVec2::new(SIZE, fmul(SIZE, -1)),
            Offset::BottomRight => FVec2::new(SIZE, SIZE),
        }
    }
}

type IndexPair = (usize, usize, Option<Offset>);

fn iter_chunk(system: &BinnedArr<usize>, coords: [usize; 2]) -> impl Iterator<Item = IndexPair> {
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

fn iter_chunks_offset(
    system: &BinnedArr<usize>,
    x: [usize; 2],
    option: [Option<Offset>; 4],
) -> impl Iterator<Item = IndexPair> {
    let coords = get_corner(x, system.side);
    (0..4).flat_map(move |i| get_chunks_iter(system, x, coords[i], option[i]))
}

fn iter_chunk_fully(
    system: &BinnedArr<usize>,
    x: [usize; 2],
    option: [Option<Offset>; 4],
) -> impl Iterator<Item = IndexPair> {
    iter_chunk(system, x).chain(iter_chunks_offset(system, x, option))
}

fn force_pair(matter: &mut Vec<Particle>, (x, y, option): IndexPair) {
    let result = match option {
        Some(offset) => matter[x].get_force_2(&matter[y], offset.to_fvec2()),
        None => matter[x].get_force(&matter[y]),
    };
    if let Some(force) = result {
        matter[x].vel -= force;
        matter[y].vel += force;
    }
}

fn force_gas(matter: &mut Vec<Particle>, system: &BinnedArr<usize>) {
    (0..system.side - 1)
        .flat_map(|i| {
            (1..system.side - 1).flat_map(move |j| {
                iter_chunk(system, [i, j]).chain(
                    get_corner_def([i, j])
                        .into_iter()
                        .flat_map(move |coords| get_chunks_iter(system, [i, j], coords, None)),
                )
            })
        })
        .chain((0..(system.side - 1)).flat_map(|i| {
            iter_chunk_fully(system, [i, 0], [None, None, None, Some(Offset::Top)]).chain(
                iter_chunk_fully(
                    system,
                    [i, system.side - 1],
                    [Some(Offset::Bottom), Some(Offset::Bottom), None, None],
                ),
            )
        }))
        .chain((1..(system.side - 1)).flat_map(|j| {
            iter_chunk_fully(
                system,
                [system.side - 1, j],
                [
                    None,
                    Some(Offset::Right),
                    Some(Offset::Right),
                    Some(Offset::Right),
                ],
            )
        }))
        .chain(iter_chunk_fully(
            system,
            [system.side - 1, 0],
            [
                None,
                Some(Offset::Right),
                Some(Offset::Right),
                Some(Offset::TopRight),
            ],
        ))
        .chain(iter_chunk_fully(
            system,
            [system.side - 1, system.side - 1],
            [
                Some(Offset::Bottom),
                Some(Offset::BottomRight),
                Some(Offset::Right),
                Some(Offset::Right),
            ],
        ))
        .for_each(|pair| force_pair(matter, pair));
}

/// [CELL, CELL + SIZE)
fn wrap_range(num: Fixed) -> Fixed {
    ((num - CELL) % SIZE).rem_euclid(SIZE) + CELL
}

fn handle_new<const X: bool>(matter: &mut Vec<Particle>, i: usize, offset: Offset) {
    let coord = if X {
        &mut matter[i].pos.x
    } else {
        &mut matter[i].pos.y
    };
    let old_value = *coord;
    *coord = wrap_range(old_value);
    if *coord == old_value {
        matter[i].draw_offset(offset.to_fvec2());
    }
}

fn fix_bounds(matter: &mut Vec<Particle>, system: &BinnedArr<usize>) {
    for k in 0..system.side {
        system.arr[[0, k]].iter().for_each(|i| {
            handle_new::<true>(matter, *i, Offset::Right);
        });
        system.arr[[system.side - 1, k]].iter().for_each(|i| {
            handle_new::<true>(matter, *i, Offset::Left);
        });
        system.arr[[k, 0]].iter().for_each(|i| {
            handle_new::<false>(matter, *i, Offset::Bottom);
        });
        system.arr[[k, system.side - 1]].iter().for_each(|i| {
            handle_new::<false>(matter, *i, Offset::Top);
        });
    }
}

fn refresh_sys(matter: &Vec<Particle>, system: &mut BinnedArr<usize>) {
    system.clear();
    for (i, mol) in matter.iter().enumerate() {
        system.add(mol.pos, i);
    }
}

fn move_gas(matter: &mut Vec<Particle>) {
    for mol in matter.iter_mut() {
        mol.move_pos();
    }
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
        move_gas(&mut matter);

        fix_bounds(&mut matter, &system);
        draw(&mut matter);

        set_default_camera();
        draw_fps();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
