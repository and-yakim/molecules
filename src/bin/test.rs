use molecules::gas::*;

fn main() {
    let number = to_fixed(100);
    println!("{}", fdiv(number, 5).to_bits() as i64 >> FRAC_BITS);
    println!(
        "{}",
        fdivf(number, to_fixed(5)).to_bits() as i64 >> FRAC_BITS
    );
}
