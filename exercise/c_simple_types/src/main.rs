// Sloppy way:
//use ding_machine::*;
// Verbose way:
//use ding_machine::ding;
//use ding_machine::on_off;
//use ding_machine::print_array;
//use ding_machine::print_difference;
//use ding_machine::print_distance;
// Explicit yet compact way:
use ding_machine::{ ding, on_off, print_array, print_difference, print_distance };

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    // 1. Pass parts of `coords` to the `print_difference` function. This should show the difference
    // between the two numbers in coords when you do `cargo run`.  Use tuple indexing.
    //
    // The `print_difference` function is defined below the `main` function. It may help if you look
    // at how it is defined.
    //
    print_difference(coords.0, coords.1); // "Difference between 6.3 and 15 is 8.7"

    // 2. We want to use the `print_array` function to print coords...but coords isn't an array!
    // Create an array of type [f32; 2] and initialize it to contain the
    // information from coords.  Uncomment the print_array line and run the code.
    //
    //let coords_arr...               // create an array literal out of parts of `coord` here
    //print_array(coords_arr);        // and pass it in here (this line doesn't need to change)
    let coords_arr = [coords.0, coords.1];
    print_array(coords_arr); // "The coordinates are (6.3, 15)"

    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"

    ding(series[6]); // "Ding, you found 13!"

    // Alternatively, since we can see that 13 is the last element, we can use
    // various tools to retrieve the last element of the array
    // Note that [`unwrap`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap)
    // simply extracts the result (`Some` value from `Option<T>`) or panics if it was `None`.
    //
    // [`std::iter::Iterator::last`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.last)
    ding(*series.iter().last().unwrap()); // "Ding, you found 13!"
    // [`std::slice::last`](https://doc.rust-lang.org/std/primitive.slice.html#method.last)
    ding(*series.last().unwrap()); // "Ding, you found 13!"

    // Better yet, we could scan/search through the array for the value using
    // [`std::iter::Iterator::find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
    ding(*series.iter().find(|&&x| x == 13).unwrap()); // "Ding, you found 13!"

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the `on_off` function the value `true` from the variable `mess`.  Done correctly,
    // `cargo run` will produce the additional output "Lights are on!" I'll get you started:
    on_off(mess.2[1].0); // "Lights are on!"

    // 5.  What a mess -- functions in a binary! Let's get organized!
    //
    // - Make a library file (src/lib.rs)
    // - Move all the functions (except main) into the library
    // - Make all the functions public with `pub`
    // - Bring all the functions into scope using use statements. Remember, the name of the library
    //   is defined in Cargo.toml.  You'll need to know that to `use` it.
    //
    // `cargo run` should produce the same output, only now the code is more organized. 🎉

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    print_distance(coords);
}
