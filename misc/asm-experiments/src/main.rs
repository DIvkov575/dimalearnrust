use std::arch::asm;
use std::ptr::write;

#[inline(never)]
fn a(a: u8) {
    let b = 1 << a;

}
#[inline(never)]
fn b(a: u8) {
    let b = 2u8.pow(5);
}


#[inline(never)]
fn c() -> u8 {
    return 5u8;
}

fn main() {
    // a(5);
    // b(5);


    // // this just gets optimized out 😥
    // let a = 0b0001;
    // let b = 0b0010;
    // let c = a | b;


    // c();

    let mask = 5u32;

    // unsafe {
    //     asm! {
    //     "mov x0, #0b0",
    //     "mov x1, #0b1",
    //     "mov x2, #0b110",
    //     "mov x3, {mask}",
    //     "and x4, x1, x2",
    //     mask = in(reg) mask,
    //     }
    // }

    unsafe {
        asm! {
            "mov x1, #0b11",
            "mov x2, #0b101",
            "mov x3, {mask}",
            "and x4, x1, x2",
            "eor x1, x4, x3",
            mask = in(reg) mask,
        }
    }

    // bp !! 😱
}

