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

#[test]
fn all_implement_register_block() {
    fn assert_register_block<B>(_block: B)
    where
        B: canadensis_node::register::RegisterBlock,
    {
    }

    assert_register_block(Empty);
    assert_register_block(NoFields {});
    assert_register_block(OneRegister {
        node_id: SimpleRegister::new("uavcan.node.id", true, true),
    });
    assert_register_block(RegistersTuple(SimpleRegister::new("test", true, true)));
}
