extern crate canadensis;

use canadensis::register::basic::SimpleRegister;
use canadensis::register::RegisterBlock;

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
