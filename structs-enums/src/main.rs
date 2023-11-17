#[derive(Debug)]
struct User {
    name: String,
    active: bool,
    email: String,
    sign_in_count: u64,
    species: Species,
}

impl User {
    fn print(&self) {
        println!(
            "Welcome to Citadel {}, your email is {} and current sign in number is {}. We fear you might be a {:?}",
            self.name, self.email, self.sign_in_count, self.species,
        );
    }

    //this is an associated function demo.
    fn usemae(
        name: String,
        active: bool,
        email: String,
        sign_in_count: u64,
        species: Species,
    ) -> Self {
        Self {
            name,
            active,
            email,
            sign_in_count,
            species,
        }
    }
}

//enums allows us to define data as being one of a possible set of values.
#[derive(Debug)]
enum Species {
    Animal,
    Human,
}

fn main() {
    let priest = User {
        name: String::from("Geoffrey"),
        active: true,
        email: String::from("trueman@gmail.com"),
        sign_in_count: 90,
        species: Species::Human,
    };
    println!("Welcome to Citadel, { }, Your email is, { }, Your account is currently,{} and this is sign in number, {}", priest.name, priest.email, priest.active, priest.sign_in_count);
    priest.print();

    let mahn = User::usemae(
        String::from("Demo"),
        true,
        String::from("Maneno"),
        9,
        Species::Animal,
    );
    mahn.print();

    let bishop = User {
        name: String::from("Mairkiti"),
        email: String::from("Marikiti@gmail.com"),
        ..priest
    };
    println!("Welcome to you too,{:#?}", bishop);

    //tuple structs
    struct Coords(i32, i32, i32);

    //match statements should be around here somewhere.
    fn matchspecies(species: Species) -> String {
        match species {
            Species::Animal => String::from("Its an animalia maayne"),
            Species::Human => String::from("It is a normal 2 walking human"),
        }
    }
    println!("This response says, {}", matchspecies(Species::Animal));
}

//structs and enums are the building blocks for creating new types in your program domain.
