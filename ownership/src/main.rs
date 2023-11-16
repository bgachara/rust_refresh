fn main() {
    let s = "Hello";
    let mut t = String::from("Bruno");

    t.push_str(", The Butcher");
    let y = t;
    println!("{s}, This is {y}");
    referensia();
    slice_demo();
}
//As opposed to the str type which is hardcoded during compilation, String is variable size.
//Drop function automatically called by the rust as soon as it goes out of scope.

fn referensia() {
    let mut c1 = String::from("Senator");

    let length = calculate_length(&mut c1);

    println!("The length of {c1} is {length}")
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", No Kugeria");
    s.len()
}

//Thou cannot have two mutable references to a mutable value.

fn slice_demo() {
    let pr = String::from("Prudence");

    let first = &pr[0..2];
    let last = &pr[4..];

    println!("This is one word, {first}, {last}");
}
