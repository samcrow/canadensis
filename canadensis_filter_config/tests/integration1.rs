extern crate canadensis_filter_config;

use canadensis_filter_config::{optimize, Filter};

#[test]
fn optimize_several_ids() {
    let interested_ids = [
        0x024F2EC8, 0x197060BA, 0x1F8FC4EB, 0x176DA287, 0x12D60349, 0x1470C4D0, 0x1CD159CA,
        0x063D5425, 0x10338C76, 0x0EA4AD64, 0x0525E1BB, 0x00942DEF, 0x0, 0x1fffffff,
    ];
    // Start with a unique filter that matches only each ID
    let filters: Vec<Filter> = interested_ids
        .iter()
        .map(|id| Filter::new(0x1fffffff, *id))
        .collect();

    for max_filters in 1..=(interested_ids.len() + 2) {
        let mut working_filters = filters.clone();
        // Optimize down to fewer filters that will accept a superset of interested_ids
        let optimized_filters = optimize(&mut working_filters, max_filters);

        for id in interested_ids.iter() {
            for filter in optimized_filters {
                println!(
                    "Optimized filter {:?} accepts {:#010x}: {}",
                    filter,
                    id,
                    filter.accepts(*id)
                );
            }
            assert!(any_accepts(&optimized_filters, *id));
        }
    }
}

fn any_accepts(filters: &[Filter], id: u32) -> bool {
    filters.iter().any(|filter| filter.accepts(id))
}
