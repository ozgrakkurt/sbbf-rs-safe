use sbbf_rs_safe::Filter;

#[test]
fn test_filter() {
    let data = (0..1024i32).collect::<Vec<_>>();

    let mut filter = Filter::new(8, data.len());

    for i in data.iter() {
        filter.insert(&i.to_be_bytes());
    }

    for i in data.iter() {
        assert!(filter.contains(&i.to_be_bytes()), "{i}");
    }
}
