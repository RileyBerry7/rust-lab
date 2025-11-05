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

    // positional args can be used by through "{index}"
    println!("\t{0}: Hi {1}, how are you?\n \
              \t{1}: I'm doing AMAZING!\n  \
              \t{0}: Well, I am glad I asked. ^^\n", "Alice", "Riley");
}
