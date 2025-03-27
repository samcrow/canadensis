extern crate canadensis_core;
extern crate canadensis_data_types;
extern crate canadensis_encoding;
extern crate heapless;

use canadensis_data_types::optimized::SubjectIdList;
use canadensis_data_types::uavcan::node::port::subject_id_1_0::SubjectID;
use canadensis_data_types::uavcan::node::port::subject_id_list_1_0::SubjectIDList as GeneratedSubjectIdList;
use canadensis_encoding::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::iter::FromIterator;

#[test]
fn same_serialization_basic() {
    check_same_serialization(&[]);
    check_same_serialization(&[1]);
    check_same_serialization(&[1, 8191]);
}

fn check_same_serialization(ids: &[u16]) {
    let generated_ids = ids.iter().map(|&id| SubjectID { value: id });
    let generated = GeneratedSubjectIdList::SparseList(heapless::Vec::from_iter(generated_ids));
    let core_ids = ids
        .iter()
        .map(|&id| canadensis_core::SubjectId::try_from(id).unwrap());
    let optimized: SubjectIdList<255> = SubjectIdList(heapless::Vec::from_iter(core_ids));

    assert_eq!(generated.size_bits(), optimized.size_bits());
    let size_bytes = generated.size_bits().div_ceil(8);
    let mut generated_bytes = vec![0u8; size_bytes];
    let mut optimized_bytes = vec![0u8; size_bytes];
    generated.serialize_to_bytes(&mut generated_bytes);
    optimized.serialize_to_bytes(&mut optimized_bytes);
    assert_eq!(generated_bytes, optimized_bytes);

    let optimized_parsed: SubjectIdList<255> =
        SubjectIdList::deserialize_from_bytes(&optimized_bytes).expect("Deserialize failed");
    assert_eq!(optimized, optimized_parsed);
}
