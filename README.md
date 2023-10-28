# python-ast-rs

A Rust library for accessing a Python AST using the Python 3 ast library. Because it relies on Python itself to parse Python, it should be very close to the reference implementation. However, it is possible that changes in the underlying Python language could prevent it from working at all if there are dramatic changes to the syntax tree.

This project is at a very early state, and should be considered completely unstable.

## Useage

Reading a Python file into an ast works like this:

```rust
use python_ast::parse;

fn read_python_file(input: std::path::Path) {
    let py = read_to_string(input).unwrap();
    let ast = parse(&py, "__main__").unwrap();

    println!("{:?}", ast);
}

```

## Notes

The goal of this project is to create a fully-compiled Python-like language that is as close to the reference language as possible, but with the advantages of Rust's static types and fearless concurrency. At this point, it's probably best to view it as a proof of concept, rather than a workable tool, but I am hoping that this will change.
