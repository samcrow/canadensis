extern crate canadensis;
extern crate canadensis_derive_register_block;

use canadensis::register::basic::SimpleRegister;
use canadensis_derive_register_block::RegisterBlock;

#[derive(RegisterBlock)]
struct Empty;

#[derive(RegisterBlock)]
struct NoFields {}

#[derive(RegisterBlock)]
struct OneRegister {
    node_id: SimpleRegister<u32>,
}

#[derive(RegisterBlock)]
struct RegistersTuple(SimpleRegister<u64>);
