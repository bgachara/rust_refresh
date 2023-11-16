const TIME_TO_TRAVEL: i8 = 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The valie of x is: {x}");
    println!("Hello, world!,{TIME_TO_TRAVEL}");

    let man: bool = true;
    let addition: f64 = 34.5 + 45.90;
    println!("Hello: {man}, {addition}");

    let tup: (i32, bool, f64) = (3, true, 89.8);
    let array = [32, 45, 67, 90];
    println!("What is {:?}", array);
    println!("This is a tuple: {:?}", tup);

    let tarray: [f32; 4] = [34.3, 45.6, 12.2, 32.4];
    println!("This is another definition, {:?}", tarray);
    mate();
    measurement(5, 's');
    let newreturn = rareturn(9);
    println!("This is the new return, {newreturn}");

    iflet();
    whilepal();
}

fn mate() {
    //show shadowing demo
    let x = 4;
    let x = x * 2;
    {
        let x = x + 30;
        println!("The value of X in here is : {x}")
    }
    println!("The value out here is: {x}")
}

// parameters vs arguments

fn measurement(height: i8, desig: char) {
    println!("Your measurements are {height}{desig}");
}

//expressions vs statements
//return values

fn rareturn(x: i8) -> i8 {
    4 + x
}

fn iflet() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The condition is, {number}");
}

fn whilepal() {
    let target = [3, 4, 5, 6, 7];

    for pro in target {
        let newnumber = pro * 2;
        println!("The target was, {newnumber}");
    }
}
