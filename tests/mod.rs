use sbbf_rs_safe::Filter;

#[test]
fn test_filter() {
    let data = (0..1024i32).collect::<Vec<_>>();

    let mut filter = Filter::new(8, data.len());

    for i in data.iter() {
        filter.insert_hash(*i as u64);
    }

    for i in data.iter() {
        assert!(filter.contains_hash(*i as u64), "{i}");
    }
}
