Attributes are for

- conditional compilaton of code
- set crate name,version and type (binary or library)
- disable lints (warnings)
- enable compiler features (macros, glob imports, etc.)
- link to a foreign library
- mark functions as unit tests
- mark functions that will be part of a benchmark

whole crate syntax  
\#![crate\_attribute]

module or item syntax  
\#[item_attribute]  (notice the missing bang ! )

various syntaxes:
- \#[attribute = "value"]
- \#[attribute(key = "value")]
- \#[attribute(value)]

```rust
#[attribute(value, value2)]

#[attribute(value, value2, value3,
            value4, value5)]
```

can be over multiple lines

