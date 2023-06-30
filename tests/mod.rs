use sbbf_rs_safe::Filter;

#[test]
fn test_filter() {
    let data = (0..1024u64).collect::<Vec<_>>();

    let mut filter = Filter::new(8, data.len());

    for i in data.iter() {
        filter.insert_hash(*i);
    }

    for i in data.iter() {
        assert!(filter.contains_hash(*i), "{i}");
    }
}

#[test]
fn test_roundtrip() {
    let data = (0..1024u64).collect::<Vec<_>>();

    let mut filter = Filter::new(8, data.len());

    for i in data.iter() {
        filter.insert_hash(*i);
    }

    let buf = filter.as_bytes();

    let other_filter = Filter::from_bytes(buf).unwrap();

    assert_eq!(other_filter.as_bytes(), buf);
}
