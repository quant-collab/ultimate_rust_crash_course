
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT:i32 = 2;

fn main() {
    let [missiles, ready] = [STARTING_MISSILES, READY_AMOUNT];
    println!("Firing {} of my {} missiles...",ready,missiles);
    println!("{} missles left",missiles - ready);
    let mut i = 0;
    let mut sum:f64 = 0.0;
    while i < 1000000 {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let y: f64 = rng.gen() ; // random number in range [0, 1)
        sum = sum + y * 52.0;
        i = i + 1;
    }
    println!("{}",(sum/1000000.0) as u8);
}
