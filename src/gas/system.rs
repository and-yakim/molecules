use super::*;

pub struct System<const R: usize> {
    pub matter: Vec<Atom<R>>,
    pub container: BinnedArr<usize>,
}

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
    Right,
    TopRight,
    BottomRight,
}

impl Offset {
    const fn to_fvec2(self, size: Fixed) -> FVec2 {
        match self {
            Offset::Top => FVec2::new(Fixed::ZERO, fmul(size, -1)),
            Offset::Bottom => FVec2::new(Fixed::ZERO, size),
            Offset::Right => FVec2::new(size, Fixed::ZERO),
            Offset::TopRight => FVec2::new(size, fmul(size, -1)),
            Offset::BottomRight => FVec2::new(size, size),
        }
    }
}

type IndexPair = (usize, usize, Option<Offset>);

fn iter_chunk(container: &BinnedArr<usize>, coords: [usize; 2]) -> impl Iterator<Item = IndexPair> {
    container.arr[coords]
        .iter()
        .enumerate()
        .flat_map(move |(i, index)| {
            ((i + 1)..container.arr[coords].len()).map(move |j| {
                let other = container.arr[coords][j];
                (*index, other, None)
            })
        })
}

fn get_chunks_iter(
    container: &BinnedArr<usize>,
    x: [usize; 2],
    y: [usize; 2],
    option: Option<Offset>,
) -> impl Iterator<Item = IndexPair> {
    container.arr[x].iter().flat_map(move |index| {
        container.arr[y]
            .iter()
            .map(move |other| (*index, *other, option))
    })
}

fn iter_chunks_offset(
    container: &BinnedArr<usize>,
    x: [usize; 2],
    option: [Option<Offset>; 4],
) -> impl Iterator<Item = IndexPair> {
    let coords = get_corner(x, container.side);
    (0..4).flat_map(move |i| get_chunks_iter(container, x, coords[i], option[i]))
}

fn iter_chunk_fully(
    container: &BinnedArr<usize>,
    x: [usize; 2],
    option: [Option<Offset>; 4],
) -> impl Iterator<Item = IndexPair> {
    iter_chunk(container, x).chain(iter_chunks_offset(container, x, option))
}

impl<const R: usize> System<R> {
    const CELL: Fixed = Atom::<R>::RC;

    fn size(&self) -> Fixed {
        self.container.size
    }

    pub fn new(size: i32) -> Self {
        let fsize = to_fixed(size); // ~32k max (with 2 * CELL)
        assert!(fmulf(fdivf(fsize, Self::CELL), Self::CELL).to_bits() == fsize.to_bits());

        let matter = Atom::<R>::generate(fsize, FVec2::new(Self::CELL, Self::CELL), 1.0);
        let container = BinnedArr::<usize>::new(fsize, Self::CELL);
        Self { matter, container }
    }

    pub fn refresh_container(&mut self) {
        self.container.clear();
        for (i, mol) in self.matter.iter().enumerate() {
            self.container.add(mol.pos, i);
        }
    }

    fn force_pair(matter: &mut Vec<Atom<R>>, (x, y, option): IndexPair, size: Fixed) {
        let result = match option {
            Some(offset) => matter[x].get_force_2(&matter[y], offset.to_fvec2(size)),
            None => matter[x].get_force(&matter[y]),
        };
        if let Some(force) = result {
            matter[x].vel -= force;
            matter[y].vel += force;
        }
    }

    pub fn force_gas(&mut self) {
        let size = self.size();
        let (matter, container) = (&mut self.matter, &self.container);
        (0..container.side - 1)
            .flat_map(|i| {
                (1..container.side - 1).flat_map(move |j| {
                    iter_chunk(container, [i, j]).chain(
                        get_corner_def([i, j]).into_iter().flat_map(move |coords| {
                            get_chunks_iter(container, [i, j], coords, None)
                        }),
                    )
                })
            })
            .chain((0..(container.side - 1)).flat_map(|i| {
                iter_chunk_fully(container, [i, 0], [None, None, None, Some(Offset::Top)]).chain(
                    iter_chunk_fully(
                        container,
                        [i, container.side - 1],
                        [Some(Offset::Bottom), Some(Offset::Bottom), None, None],
                    ),
                )
            }))
            .chain((1..(container.side - 1)).flat_map(|j| {
                iter_chunk_fully(
                    container,
                    [container.side - 1, j],
                    [
                        None,
                        Some(Offset::Right),
                        Some(Offset::Right),
                        Some(Offset::Right),
                    ],
                )
            }))
            .chain(iter_chunk_fully(
                container,
                [container.side - 1, 0],
                [
                    None,
                    Some(Offset::Right),
                    Some(Offset::Right),
                    Some(Offset::TopRight),
                ],
            ))
            .chain(iter_chunk_fully(
                container,
                [container.side - 1, container.side - 1],
                [
                    Some(Offset::Bottom),
                    Some(Offset::BottomRight),
                    Some(Offset::Right),
                    Some(Offset::Right),
                ],
            ))
            .for_each(|pair| Self::force_pair(matter, pair, size));
    }

    pub fn move_gas(&mut self) {
        for mol in self.matter.iter_mut() {
            mol.move_pos();
        }
    }

    /// [CELL, CELL + SIZE)
    fn wrap_range(num: Fixed, size: Fixed) -> Fixed {
        ((num - Self::CELL) % size).rem_euclid(size) + Self::CELL
    }

    pub fn fix_bounds(&mut self) {
        let size = self.size();
        let (matter, container) = (&mut self.matter, &self.container);
        for k in 0..container.side {
            container.arr[[0, k]]
                .iter()
                .chain(container.arr[[container.side - 1, k]].iter())
                .for_each(|i| {
                    matter[*i].pos.x = Self::wrap_range(matter[*i].pos.x, size);
                });
            container.arr[[k, 0]]
                .iter()
                .chain(container.arr[[k, container.side - 1]].iter())
                .for_each(|i| {
                    matter[*i].pos.y = Self::wrap_range(matter[*i].pos.y, size);
                });
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        self.matter.iter().for_each(|atom| atom.draw(d));
    }
}
