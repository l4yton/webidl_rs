# webidl_rs
A [Web IDL](https://webidl.spec.whatwg.org/) parser for rust, powered by [nom](https://github.com/Geal/nom). It supports converting parsed Web IDL back to a string.

## Usage

### `Cargo.toml`
```toml
[dependencies]
webidl_rs = { git = "https://github.com/l4yton/webidl_rs" }
```

### `src/main.rs`
```rust
use webidl_rs::{Constructor, Definition, Member};

fn main() {
    let mut definitions =
        webidl_rs::parse("[Exposed=Window] interface Foo { };").unwrap();

    // Add a constructor to the first definition.
    if let Some(Definition::Interface(interface)) = definitions.first_mut() {
        interface.members.push(Member::Constructor(Constructor {
            ext_attrs: vec![],
            arguments: vec![],
        }))
    }

    // Print the Web IDL with the added constructor.
    print!("{}", webidl_rs::to_string(&definitions));
}
```

## TODO
- [ ] Better documentation
- [ ] Add more tests
- [ ] Replace asserts with custom errors in parser
- [ ] Validate Web IDL semantically (more)
