//variables are by default always immutable)

fn main() {
    //not allowed and will not compile since x is immutable,
    //therefore no other value can be assigned! x must be mut!

    //let x = 5;
    //println!("Der Wert x ist: {}", x);
    //x = 6;
    //println!("Der Wert x ist {}", x);

    //correct way:

    let mut x = 5;
    println!("The variable x has the value: {}", x);

    x = 6;
    println!("The variable x has the value: {}", x);

    //there are also constants which never can be mutable#
    //keyword const instead of let
    //type must be annotated always!
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //Shadowing can be done by using let twice with same variable name
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value within this scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    //advantage of shadowing compared to mut: shadowing muts variable just once or just for a specific scope
    //mut gives possibility to change value at any position in the program
    //and readability through using of same variable name instead of assigning value to a new one (especially cool for parsing)

    let spaces = "  ";
    let spaces = spaces.len(); //works because of shadowing(using let will create new variable with same name)
    println!("{}", spaces);

    //let mut spaces = "  ";
    //spaces = spaces.len(); //does not work with mut since you cant mut the type of a variable

    //=========Datatypes=============
    //rust has a static type system -> all types must be known to compile time
    //when parsing an string to int, the type int must be declared
    let example: u32 = "42".parse().expect("Not a number");

    //skalar types (ints, floats, bools, chars)
    //binding types (tupel, arrays)

    let tup = (500, 6.4, 1);
    //deconstructing of tupel to access values
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    //another way to seperately acess values in tupel:
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //array with same value
    let b = [3; 5];
    //access elements
    let first = b[0];
}
