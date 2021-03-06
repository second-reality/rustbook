const A: u32 = 43;

fn compute(x: u32) -> u32 {
    let res = { x + A };
    res + 3
}

mod other {

    #[derive(Debug)]
    // pub to make access to Blop out of module
    pub struct Blop {
        field_1: u32,
        field_2: u32,
    }

    impl Blop {
        pub fn new(val: u32) -> Self {
            Blop {
                field_1: val,
                field_2: val,
            }
        }
    }
}

fn iterate_array() {
    let mut a = [10, 20, 30, 40, 50];

    for val in a.iter_mut() {
        *val += 1;
        println!("{}", val);
    }

    for idx in 0..a.len() {
        println!("{}", a[idx]);
    }
}

fn slice_try(s: &str) {
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

    let blop = other::Blop::new(3);
    // forbids explicit construction because struct has private field in another module!
    // If in same module, it is accessible
    // let blop2 = Other::Blop { field_1: 0, field_2: 0 };

    println!("{:?}", blop);

    let toto = String::from("trying");
    let tata = String::from("tata");
    // +, takes ownership of string toto, and add tata to it
    // thus, toto is accessible after, so it is efficient (no useless copy needed)
    let blop = toto + " " + &tata;

    println!("{}", blop);

    // format macro does not take ownership of strings!
    let blop_format = format!("{} | {} | {}", tata, blop, blop);

    println!("{}", blop_format);

    //let hello = "Здравствуйте";
    //let s = &hello[0..3]; // runtime check for char border when using slice
    // that is why rust forbids direct access to memory: hello[2]

    // replace part of string
    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(0); // do not replace anything if not found

    // Replace the range up until the β from the string
    s.replace_range(..beta_offset, "that is nice ");
    assert_eq!(s, "that is nice β is beta");

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    let val = scores.get(&String::from("Yellow"));
    println!("{}", val.unwrap()); // unwrap is nice while you develop, you can deal with errors after :)
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", &scores);
}
