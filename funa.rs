use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut mass_grams = 1_000_000_000_000_000.0 / (24_000_000_000.0 * 0.35);

    for month in 1..=12 {
        print!("month {} - decay: {:.2} grams\r", month, mass_grams);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        mass_grams -= mass_grams / 12.0;

        sleep(Duration::from_secs(1));
    }
    println!();
}
