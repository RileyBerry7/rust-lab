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


/* ------------------------------ debugging ------------------------------
    types in the std library are automatically implmented into std::fmt.
    any other types require an implmenatation to be formatted using std::fmt.
    
    fmt::debug   - all types are automatically implemented
    fmt::display - non-std library types require manual implmenation
    
    this structure will not print with either:
    struct unprintable(i32);
    
    the 'derive' attribute is what auto creates the implmenation for our struct:
    #[derive(debug)]
    struct debugprintable(i32); 
    
    all std library types are printable with '{:?}' too.
    
-----------------------------------------------------------------------------------------------------------------*/


/* ------------------------------ Primitives -----------------------------
   Rust provieds access to a wide variety of primitives.

    Scalar Types:
        - signed integers
        - unsigned integers
        - floating point
        - char Unicode scalar values
        - bool either true or false
        - The unit type (), whose only value is an empty tuple ()

    Compound Types:
        - arrays: [1, 2, 3]
        - tuples: (1, a)

Variables can always be type annotated. Numbers may additionally be annotated via suffix or by default. Integer 
defaults to i32 and floats to f64. Note that Rust can also infer types from context.
-----------------------------------------------------------------------------------------------------------------*/
pub fn primitives() {
    // Variables can be type annotated
    let logical: bool = true;
    let a_float: f64  = 1.0; // regular annotation
    let an_int        = 5i32; // suffix annotation

    // Without annotations, 'default' types are used
    let default_float = 3.0;
    let default_int   = 7;

    // A type can also be inferred from context
    let mut inferred_type = 12;  // <- this type is set as i64 because of the next line.
    inferred_type = 4294967296i64;

    // mut makes a variable mutable; its value can change.
    
    // Error: variable types cant be changed.
    let mutable_var = 25;
    // mutable_var = true; // ERROR
    // However: variables can be overwritten with shadowing (essentially a redeclaration)
    let mutable_var = true;

// ------------ Compound Types -------------
    // Array signature consists of a type T and a length: [T; length]
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // A Tuple is an ordered list of heterogenious types: ()
    let my_tuple = (5u32, 1u8, true, -5.34f32);

}
