use molecules::gas::*;
use molecules::*;

fn main() {
    init_rand();
    let instant = time::Instant::now();

    let mut system = System::<4>::new(8000);

    println!("N: {:.1}M", system.matter.len() as f32 / 1_000_000f32);
    println!("Init: {} ms", instant.elapsed().as_millis());

    let mut frame = time::Instant::now();
    while instant.elapsed().as_secs() < 5 {
        system.refresh_container();
        system.force_gas();
        system.move_gas();
        system.fix_bounds();

        println!("{} ms", frame.elapsed().as_millis());
        frame = time::Instant::now();
    }
}
