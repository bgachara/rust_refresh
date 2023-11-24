use std::collections::HashMap;

//start off with Vec<T>, more than one value in a ds, but of the same type.
fn main() {
    let new: Vec<i32> = Vec::new();
    //use the macro
    let mut new = vec![2, 3, 5];

    new.push(4);
    new.push(6);
    new.push(7);

    for i in &new {
        println!("{i}");
    }
    println!("Hello, world, this vector is {:?}", new);

    //string maneno below

    let mut s = String::from("footman");
    s.push_str("bar");
    for q in s.chars() {
        println!("{q}");
    }
    for r in s.bytes() {
        println!("{r}");
    }

    println!("This is the string,{:?}", s);

    //hashmaps...

    let mut pluto = HashMap::new();

    pluto.insert("name", "Kevin");
    pluto.insert("age", "Manu");

    println!("Imma try this set, {:?}", pluto);
}
