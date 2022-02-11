use kwindex::*;

#[test]
fn extend_example() {
    let kwindex = KWIndex::new().extend_from_text("It ain't over untïl it ain't, over.");
    //No other way to examine the contents of the inner tuple
    assert_eq!(
        "KWIndex([\"It\", \"over\", \"untïl\", \"it\", \"over\"])",
        format!("{:?}", kwindex)
    );
}

#[test]
fn test_empty() {
    let mut kwindex = KWIndex::new();
    assert_eq!(true, kwindex.is_empty());
    kwindex = kwindex.extend_from_text("Hel-lo wor-ld");
    assert_eq!(true, kwindex.is_empty());
    kwindex = kwindex.extend_from_text("Hello world");
    assert_eq!(false, kwindex.is_empty());
}

#[test]
fn extend_example_count_matches() {
    let kwindex = KWIndex::new().extend_from_text("It ain't over untïl it ain't, over.");
    //No other way to examine the contents of the inner tuple
    assert_eq!(2, kwindex.count_matches("over"));
    assert_eq!(1, kwindex.count_matches("It"));
    assert_eq!(1, kwindex.count_matches("it"));
    assert_eq!(0, kwindex.count_matches("ain't"));
    assert_eq!(1, kwindex.count_matches("untïl"));
}

#[test]
fn nth_uppercase_workout() {
    let kwindex = KWIndex::new().extend_from_text("I am THE TH-E WALRUS WALRUS.");
    //No other way to examine the contents of the inner tuple
    assert_eq!("I", kwindex.nth_uppercase(0).unwrap());
    assert_eq!("THE", kwindex.nth_uppercase(1).unwrap());
    assert_eq!("WALRUS", kwindex.nth_uppercase(2).unwrap());
    assert_eq!("WALRUS", kwindex.nth_uppercase(3).unwrap());
    assert_eq!(None, kwindex.nth_uppercase(4));
}
