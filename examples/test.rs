use vecxd::VecXD;

fn main() {
    let mut a = VecXD::new([1.0, 2.0]);
    let b = VecXD::new([1.0, 2.0]);

    a *= 1.2;
    a += b;
    a -= b * 0.2;

    let c = a * b;

    println!("{:?}, {:?}, {:?}", a, b, c);
}
