# G6

Pure rust parser for `graph6`, `digraph6` and `sparse6` formats.

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

- import undirected graph6 from text

```rust
#[test]
fn import_digraph6() {
    let digraph = DiGraph6::from_str("&B|o").unwrap();
    assert_eq!(digraph.nodes(), 3);
    assert_eq!(digraph.edges(), 7);
}
```

- import large sparse6 from text

```rust
#[test]
fn import_sparse6() {
    let digraph = Sparse6::from_str(":Fa@x^").unwrap();
    assert_eq!(digraph.nodes(), 7);
    assert_eq!(digraph.edges(), 4);
}
```

## Export

- to text format

todo

- to wolfram mathematica

todo

## Test Cases

http://users.cecs.anu.edu.au/~bdm/data