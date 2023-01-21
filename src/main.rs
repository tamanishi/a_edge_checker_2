use proconio::input;

fn main() {
    input! {
        a:u32,
        mut b:u32,
    }

    if b % 2 == 1 {
        b -= 1;
    }

    match b / 2 == a {
        true => println!("Yes"),
        false => println!("No"),
    }
}
