const STARTING_MISSILES: i32 = 8;

fn fire(n: i32, available: &mut i32) {
    if n < 1 {
        println!("Can't fire less than 1 missile, silly");
        return;
    }
    if n < *available {
        println!("Firing {} of my {} missiles...", n, available);
        *available -= n;
    } else if *available > 0 {
        println!("Request to fire {} missiles but only have {} missiles (firing those)", n, available);
        *available = 0;
    } else {
        println!("Can't fire {} missiles because we're out of ammo :'(", n);
    }
}

fn main() {
    let mut available: i32 = STARTING_MISSILES;
    fire(-9, &mut available);
    fire(4, &mut available);
    fire(1, &mut available);
    fire(100, &mut available);
    fire(2, &mut available);
}
