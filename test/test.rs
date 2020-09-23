fn have_three() -> u16 {
    3
}

fn main() {
    fn have_two() -> u16 {
        return 2;
    }

    let k = have_two();
    let j = have_three();
    let i = 0;
    let a = Some(1);

    let z = j + k;
    println!("{:?}", z);
}
