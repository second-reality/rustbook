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
    let mut a = [10, 20, 30, 40, 50];

    for val in a.iter_mut() {
        *val += 1;
        println!("{}", val);
    }

    for idx in 0 .. a.len() {
        println!("{}", a[idx]);
    }
}

fn slice_try(s: &str)
{
    println!("{}", s);
}

fn main() {
    let mut x = 3;

    let number = if compute(x) > 3 { 5 } else { 6 };

    x = number;

    slice_try("Good");
    let mut try_str = String::from("Good from strings too");
    slice_try(&mut try_str);
    //let other = try_str; // Forbidden, because will make next line illegal (moved try_str)
    slice_try(&mut try_str[2..6]);

    iterate_array();

    println!("x {}", x);
}
