const A: u32 = 43;

fn compute(x: u32) -> u32
{
    let res = {
        x + A
    };
    res + 3
}

fn main() {
    let mut x = 3;

    let number = if compute(x) > 3 { 5 } else { 6 };

    x = 3;
}
