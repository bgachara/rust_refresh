mod cake {
    pub mod flavors {
        pub const COCO: &str = "Coco Bands";
    }
    pub mod toppings {
        pub const TOPS: &str = "Vanilla tings";
    }
}

fn main() {
    let name = "Brian Geoffrey";

    let favorite = "Purple";
    println!("Hello {:?}", favorite);

    let favorite = "Indigo";
    println!("Hello {:?}", favorite);

    let printman = || println!("{name}");
    println!("Hello {:?}", name);
    printman();
    println!("{}", cake::flavors::COCO);
    println!("{}", cake::toppings::TOPS);
}
// Remember the distinction between expressions and statements, Rust is an expression based language, all code blcoks return a value unless explicitly declared.
// Rust has stack-based types, i.e i64 and heap-based types, i.e String and both are treated differently.
// Referencing allows us to reuse data without needing to allocate more memory, &data, dereference with *data. automatic dereferencing.
// Mutability, ability to variable's value to be changed in memory, references can be mutable too,
// Constant functions.
// Modules, modname::modfunction.
// crate::, self::, super::, you can also rename imports.
// Macros is a way of manipulating and generating code, generate and expand raw source code, attributes(inner, outer,) and macros.
