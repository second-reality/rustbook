const A: u32 = 43;

fn compute(x: u32) -> u32
{
    let res = {
        x + A
    };
    res + 3
}

fn iterate_array()
{
    let a = [10, 20, 30, 40, 50];

    for val in a.iter() {
        println!("{}", val);
    }
}

fn main() {
    let mut x = 3;

    let number = if compute(x) > 3 { 5 } else { 6 };

    x = number;

    iterate_array();

    println!("x {}", x);
}
