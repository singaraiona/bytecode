#![feature(const_fn,drop_types_in_const)]
pub mod bc;
use bc::*;

fn main() {
    let ref mut h = handle();
    let bc = ByteCode::Call(ByteCode::Call(ByteCode::Symbol(1).bin(h),
                                           ByteCode::Number(12).bin(h))
                                .bin(h),
                            ByteCode::Number(123).bin(h));
    println!("ByteCode: {:?}", bc);
}
