use g6::DiGraph;

#[test]
fn test_header() {
    let repr = include_str!("digl3.d6");
    assert!(DiGraph::from_d6(repr).is_ok());
}