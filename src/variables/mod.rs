pub fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}

pub fn shadowing() {
    let x = 5;

    let x = x + 1; // シャドーイング

    { // これだけでスコープになる
        let x = x * 2; // シャドーイング
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}