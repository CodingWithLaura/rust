//rust uses snake_case for function names
fn main() {
    println!("This is not a function I wrote");
    real_function();
    function_with_param(5);
}

fn real_function() {
    println!("But this is a function I wrote :)")
}

fn function_with_param(x: i32) {
    println!("The value of the param is {}", x);
}
