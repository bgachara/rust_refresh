//use generics to create definitions for items like function signatures or structs, which we can them use with many different concrete data types.
struct Point<T> {
    x: T,
    y: T,
}

//methods on the generic structs

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Location<T, U> {
    estate: T,
    street: U,
}

//also works with enums

enum Option<T> {
    Some(T),
    None,
}

//also come across enums with different types

enum Options<T, E> {
    Some(T),
    None(E),
}

fn main() {
    let number_list = vec![23, 34, 45, 56, 67];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let character_list = vec!['e', 'r', 't', 'y', 'l'];

    let result = largest(&character_list);
    println!("The largest character is {}", result);

    println!("Hello, world!");
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//Use traits to define shared behaviour
//different types share behaviour if we can call the same methods on them.

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Location<T, U> {
    fn summarize(&self) -> String {
        format!("{}:{}", self.x, self.y)
    }
}

//traits as paramaters

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//traits bounds syntactic sugar

pub fn notifys<T: Summary>(item: &T) {}

//each have their own pros and cons.
//You can also use Trait bounds with where clause

//Lifetimes
//describe relationships between references without looking at their scopes so to say
//can never be one as it is supposed to show relationship between references.
//lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
//i.e returned reference will be valid as long as both parameters are valid

fn lifetime<'a>(item: &'a str, name: &'a str) -> &'a str {}
