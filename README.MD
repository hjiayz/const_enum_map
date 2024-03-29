# const_enum_map


Get const value from enum handle

## Examples

```rust
use const_enum_map::{ConstKey,const_enum_map};

fn item0() -> &'static str {
    "item0"
}
fn item1() -> &'static str {
    "item1"
}
const_enum_map! {
    Foo => fn()->&'static str,
    Item0 => item0,
    Item1 => item1
}

fn main() {
    assert_eq!(Foo::Item0.get()(), "item0");
    assert_eq!(Foo::Item1.get(), Foo::Item1.get());
    assert_eq!(Foo::value_list(), Foo::value_list());
}
```

License: MIT
