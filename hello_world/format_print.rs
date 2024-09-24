fn main() {
    // '{}' will be replaced by the argument
    println!("{} days", 31);

    /* Positional arguments can be used. Specifying an integer inside '{}'
     * determines which additional argument will br replaced.
     * Arguments start at 0 immediately after the format string.
     */
    println!("{0}, this is {1}. {1}, this is {0}", "Vikas", "Aman");

    // Can pass named arguments as well.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    /* Different formatting can be invoked by specifying the foramt character
     * after a ':'.
     */
    println!("Base 10: {}", 1234);
    println!("Base 2: {:b}", 1234);
    println!("Base 8: {:o}", 1234);
    println!("Base 16: {:x}", 1234);

    // right-justify text
    println!("{number:>5}", number=1);

    // pad numbers with extra zeros
    println!("{number:0>5}", number=1);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    // only types that implement fmt::Display can be formated with '{}'.
    // User-defined types do not implement fmt::Display by default.

    //#[allow(dead_code)]
    //struct Structure(i32);

    // println!("This struct '{}' won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
