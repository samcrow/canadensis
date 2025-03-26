extern crate canadensis;

use canadensis::register::basic::SimpleRegister;
use canadensis::register::{Register, RegisterBlock};

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
fn test_empty_no_fields() {
    let mut empty = Empty;
    assert!(empty.register_by_index(0).is_none());
    assert!(empty.register_by_index_mut(0).is_none());
    assert!(empty.register_by_name_mut("lobster_thermidor").is_none());

    let mut no_fields = NoFields {};
    assert!(no_fields.register_by_index(0).is_none());
    assert!(no_fields.register_by_index_mut(0).is_none());
    assert!(no_fields
        .register_by_name_mut("lobster_thermidor")
        .is_none());
}

#[test]
fn test_one_register_named() {
    let mut block = OneRegister {
        node_id: SimpleRegister::new("node_id", true, true),
    };
    let node_id_ptr = &block.node_id as &dyn Register as *const _;

    assert_eq!(
        node_id_ptr,
        block.register_by_index(0).expect("No register") as *const _
    );
    assert_eq!(
        node_id_ptr,
        block.register_by_index_mut(0).expect("No register") as *const _
    );
    assert_eq!(
        node_id_ptr,
        block.register_by_name_mut("node_id").expect("No register") as *const _
    );
}

#[test]
fn test_one_register_tuple() {
    let mut block = RegistersTuple(SimpleRegister::new("cans_of_spam", true, true));
    let node_id_ptr = &block.0 as &dyn Register as *const _;

    assert_eq!(
        node_id_ptr,
        block.register_by_index(0).expect("No register") as *const _
    );
    assert_eq!(
        node_id_ptr,
        block.register_by_index_mut(0).expect("No register") as *const _
    );
    assert_eq!(
        node_id_ptr,
        block
            .register_by_name_mut("cans_of_spam")
            .expect("No register") as *const _
    );
}
