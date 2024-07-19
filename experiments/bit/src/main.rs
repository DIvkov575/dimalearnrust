fn main() {
    let A = 0b00000001;
    let B = 0b00000010;
    let C = 0b00000100;

    let mask = 5;

    let ABC = (A | B | C) & mask ^ 255;

    println!("{:b}", ABC);
    // println!("{:b}", 255);
}
