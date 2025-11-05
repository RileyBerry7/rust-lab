// basics.rc

/* ------------------------------ Types of Comments ------------------------------

// Line Comment

/*
* Block Comment (interior column of * is not necessary)
*/

/// Doc Comment (following item)

//! Doc Comment (enclosing item)


//* <- add another '/' before the 1st one to uncomment the whole block
println!("This will now work");

// [Line comments remain commented] 
*/

// println! is a macro that prints text to the console. */

/* ------------------------------ Formatted Printing ------------------------------
Formatted print
Printing is handled by a series of macros defined in std::fmt some of which are:

format!: write formatted text to String
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as print! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint! but a newline is appended.
All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time. 
-----------------------------------------------------------------------------------------------------------------*/

pub fn formatted_printing() {

    // In general, the "{}" will be replaced with any args. (casted to_str)
    println!("{} days until christmas!!!\n", 51);

    // positional args can be used by through "{index}" or "{arg_name}"
    println!("\t{0}: Hi {1}, how are you?\n \
              \t{1}: I'm doing AMAZING!\n   \
              \t{0}: Well, I am glad I asked. ^^\n", "Alice", "Riley");

    // pos_args also work with named args
    println!("{subject} {verb} {object}\n", object="the lazy dog",
                                          subject="the quick brown fox",
                                          verb="jumps over");

    // args can be specifically formatted be using "{:modifer}"
    // {:b}, int_arg); <- binary
    // {:o}, int_arg); <- octal
    // {:h}, int_arg); <- hexadecimal
    
    // You can Left or Right-Justify like so:
    println!("|{arg:>15}|", arg="argument"); // Note: justify num counts the arg chars

    // You can also add non ws padding like so:
    println!("|{arg:_<15}|", arg="text");

    // You can use named arguments in the format specifier by appending a `$`.
    println!("|{bin_num:0>bits$}|", bin_num=1110010, bits=15); // <- lil-endian
    
    // Rustc will throw an error if count(used_args) > count(declared_args)
    println!("My name is {0}, {1} {0}.", "Bond", "James");

    // Only types implemented with fmt::Display can be formatted with '{}'. User defined types dont
    // implment fmt::Display by default.

    // std::fmt contains many 'traits' which govern how text is displayed. The base form of 2
    // important 'traits are listed below:
    //  - fmt::Debug    {:?}  - Format for debugging purposes
    //  - fmt::Display  {}    - Format for elegant, user-friendly text
}


/* ------------------------------ Formatted Printing ------------------------------

-----------------------------------------------------------------------------------------------------------------*/
