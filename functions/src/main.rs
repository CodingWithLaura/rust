//rust uses snake_case for function names
fn main() {
    println!("This is not a function I wrote");
    real_function();
    function_with_param(5);
    let y = 6; //this is also a statement, needs to be within a function
}

fn real_function() {
    println!("But this is a function I wrote :)")
}

//using params one have to declare the typ of the param!
fn function_with_param(x: i32) {
    println!("The value of the param is {}", x);
}

//statement: instruction which returns no value
//expression: instruction which always returns a value

//let a = (let b = 6); ->
//this will not work since let b = 6 is a statement and returns no value
//for a to bind against
