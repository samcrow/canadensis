extern crate canadensis_derive_register_block;
extern crate canadensis_node;

use canadensis_derive_register_block::RegisterBlock;
use canadensis_node::register::basic::SimpleRegister;

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
