# G6

Pure rust parser for graph6, digraph6 formats.

## Import

- import undirected graph6 from text

```rust
#[test]
fn import_graph6() {
    let graph = Graph6::from_str("E?Bw").unwrap();
    assert_eq!(graph.nodes(), 5);
    assert_eq!(graph.edges(), 6);
}
```

- import undirected graph6 from file

```rust
#[test]
fn import_digraph6() {
    let digraph = DiGraph6::from_str("&B|o").unwrap();
    assert_eq!(digraph.nodes(), 3);
    assert_eq!(digraph.edges(), 7);
}
```

## Export

- to text format

- to wolfram mathematica

## Test Cases

http://users.cecs.anu.edu.au/~bdm/data