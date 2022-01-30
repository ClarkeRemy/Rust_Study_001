### __Keywords Currently in Use__

The following keywords currently have the functionality described.
- `as` - perform primitive casting,
 disambiguate the specific trait containing an item,
 or rename items in `use` and `extern crate` statements
- `async` - return a `Future` instead of blocking the current thread
- `await` - suspend execution until the result of a `Future` is ready
- `break` - exit a loop immediately
- `const` - define constant items or constant raw pointers
- `continue` - continue to the next loop iteration
- `crate` - link an external crate or a macro variable representing the crate in which the macro is defined
- 'dyn` - dynamic dispatch to a trait object
- `else` - fallback for if and if let control flow constructes
- `enum` - define an enumeration
- `extern` - link an exernat crate,
 function,
 or variable
- `false` - Boolean false literal
- `fn` - define a function or the fuction pointer type
- `for` - loop over items from an iterator,
 implement a trait,
 or specify a higher-ranked lifetime
- `if` - branch based on the result of a conditional expression
- `impl` - implement inherent or trait functionality
- `in` - part of `for` loop syntax
- `let` - bind a variable
- `loop` - loop unconditionally
- `match` - match a value to patterns
- `mod` - define a module
- `move` - make a closure take ownership of all it's captures
- `mut` - denote public visibility in struct fields,
 `impl` blocks,
 or modules,
- `ref` - bind by reference
- `return` - return from function
- `Self` - a type alias for the type we are defining or implementing
- `self` - method subject or current module
- `static` - global variable or lifetime lasting the entire program execution
- `struct` - define a structure
- `super` - parent module of the current module
- `trait` - define a trait
- `true` - Boolean true literal
- `type` - define a type alias or associated type
- `union` - define a union and is only a keyword when used in a union declaration
- `unsafe` - denote unsafe code,
 functions,
 traits,
 or implementations
- `use` - bring symbols into scope
- `where` - denote clauses that constrain a type
- 'while' - loop conditionally based on the result of an expression

### __Keywords Reserved for Future Use__

The following keywords do not have any fuctionality, but are reserved by Rust for potential use.
- `abstract`
- `become`
- `box`
- `do`
- `final`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yeild`

### __Raw Identifiers__

Raw *identifiers* are the syntax that lets you use keywords where they wouldn't normally be allowed. You use a raw identifier by prefixing a keyword with `r#`.

For example,
`match` is a keyword.
If you try to compile the following fuction that uses `match` as its name:

Filename: src/main.rs
```rust
fn match(needle: &str,haystack: &str) -> bool {
    haystack.contains(needle)
}
```
you'll get this error
```rust
error: expected identifier, found keyword `match`
--> src/main.rs:4:4
  |
4 | fn match(needle: 6str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```
The error shows that you can't use the keyword `match` as the fuction identifier. To use `match` as a fuction name, you need to use the raw identifier syntax, like this :

Filename: src/nain.rs
```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo","foobar"));
}
```