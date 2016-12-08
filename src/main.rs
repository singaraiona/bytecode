#![feature(const_fn,drop_types_in_const)]
pub mod bc;
use bc::*;

fn main() {
    init_pool();
    let bc = ByteCode::Call(ByteCode::Call(ByteCode::Symbol(1).bin(), ByteCode::Number(12).bin())
                                .bin(),
                            ByteCode::Number(123).bin());
    println!("ByteCode: {:?}", bc);
}
