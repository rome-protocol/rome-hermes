use af_utilities::types::{I256, IFixed};

#[expect(clippy::unwrap_used)]
fn main() {
    let min = i128::MIN;
    println!("{min}");
    let i256 = I256::from(min);
    println!("{i256}");
    let value = IFixed::from(i128::MIN);
    println!("{value}");

    println!("{}", 3 % 2);
    println!("{}", -3 % 2);
    println!("{}", 3 % -2);
    println!("{}", -3 % -2);
    println!("{}", I256::from(3) % 2.into());
    println!("{}", I256::from(-3) % 2.into());
    println!("{}", I256::from(3) % (-2).into());
    println!("{}", I256::from(-3) % (-2).into());

    let x: IFixed = 50.50.try_into().unwrap();
    let y: IFixed = 8.125.try_into().unwrap();
    println!("{x} / {y} = {}", x / y);
    println!("({x} / {y}).integer() = {}", (x / y).integer());
    println!("({x} / {y}).trunc() = {}", (x / y).trunc());
    let remainder = x % y;
    // The answer should be 1.75
    println!("{x} % {y} = {remainder}");
}
