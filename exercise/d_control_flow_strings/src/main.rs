// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        // 1a. Your task: handle the command-line arguments!
        //
        // - If arg is "sum", then call the sum() function
        // - If arg is "double", then call the double() function
        // - If arg is anything else, then call the count() function, passing "arg" to it.
        if arg == "sum" {
            println!("sum");
            sum();
        } else if arg == "double" {
            println!("double");
            double();
        } else {
            println!("count");
            count(arg);
        }

        // 1b. Now try passing "sum", "double" and "bananas" to the program by adding your argument
        // after "cargo run".  For example "cargo run sum"
        // input: `1 2 double sum sum`
        // output:
        // count
        //
        // count
        //
        // double
        // You can double x 0 times until x is larger than 500
        // sum
        // The sum is 0
        // sum
        // The sum is 0
    }
}

fn sum() {
    let mut sum = 0;
    // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
    // and add them all together (increment the `sum` variable).  Hint: You should get 255
    // Run it with `cargo run sum`
    for i in 7..=23 {
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    const TARGET: i32 = 1024;
    let mut count = 0;
    let mut x = 1;
    // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
    // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
    // with `cargo run double`  Hint: The answer is 9 times.
    while x <= TARGET {
        count += 1;
        x *= 2;
    }

    println!("You can double x {} times until x is larger than {}", count, TARGET);
    //println!("...log2({}) = {}", TARGET, numeric::math::log2(TARGET));
    println!("Note: floor(log2({})) + 1 = {}", TARGET, log_2(TARGET) + 1);
}

// From https://users.rust-lang.org/t/logarithm-of-integers/8506/3
const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

fn log_2(x: i32) -> u32 {
    assert!(x > 0);
    num_bits::<i32>() as u32 - x.leading_zeros() - 1
}

fn count(arg: String) {
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    //
    // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
    let mut remaining = 8;
    loop {
        println!("{}", arg);
        remaining -= 1;
        if remaining <= 0 {
            break;
        }
    }
    println!(); // This will output just a newline at the end for cleanliness.
}
