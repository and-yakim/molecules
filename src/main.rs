use molecules::gas::*;
use molecules::*;

fn main() {
    init_rand();
    let instant = time::Instant::now();

    let mut system = System::<4>::new(8000);

    println!("N: {:.1}M", system.matter.len() as f32 / 1_000_000f32);
    println!("Init: {} ms", instant.elapsed().as_millis());

    let mut frame = time::Instant::now();
    let mut timings = [0u128; 4];
    while instant.elapsed().as_secs() < 5 {
        system.refresh_container();
        let elapsed0 = frame.elapsed().as_micros();
        timings[0] = elapsed0;
        system.force_gas();
        let elapsed1 = frame.elapsed().as_micros();
        timings[1] = elapsed1 - elapsed0;
        system.move_gas();
        let elapsed2 = frame.elapsed().as_micros();
        timings[2] = elapsed2 - elapsed1;
        system.fix_bounds();
        let elapsed3 = frame.elapsed().as_micros();
        timings[3] = elapsed3 - elapsed2;

        println!("{} ms, {:?} µs", frame.elapsed().as_millis(), timings);
        frame = time::Instant::now();
    }
}

// Vec      ~ 2400µs
// SmallVec ~ 3400µs
// ArrayVec ~ 2900µs
