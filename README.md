# Turing Machine

Turing Machine Simulation Library written in Rust.

## Example

```
$ cargo run --example [EXAMPLE_NAME]
```

### anbn

$L=\lbrace a^nb^n: n \geq 1 \rbrace$

```
$ cargo run --example anbn
aabb
Accepted

$ cargo run --example anbn
aab
Not Accepted
```