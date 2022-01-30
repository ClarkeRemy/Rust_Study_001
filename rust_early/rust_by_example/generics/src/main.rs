// A concrete type `A`.
struct A;

// In defining the type `S`, the first use of `A` is not preceded by `<A>`.
// Therefore, `S` is a concrete type, and `A` is defined as above.
struct S(A); // Concrete type `S`.
             //            ^ Here is `S`s first use of the type `A`.

// Here, `<T>` precedes the first use of `T`, so `SGen` is a generic type.
// Because the type parameter `T` is generic, it should be anything, including
// the concrete type `A` defined at the top.
struct SGen<T>(T); //generic type `SGen`

// The following function `reg_fn` that takes an argument `_s` of type `S`.
// This has no `<T>` so this is not a generic function.
fn reg_fn(_s: S) {}

// Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
// It has been explicitly given the type parameter `A`, but because `A` has not
// been specified as a generic type parameter for `gen_spec_t`, it is not generic.
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // `S` is concrete and explicitly takes `A`.
    let _s = S(A);

    // Create a variable `_char` of type `SGen<char>`
    // and give it the value `SGen('a')`.
    // Here, `SGen` has a type parameter explicitly specified.
    let _char: SGen<char> = SGen('a');

    // `SGen` can also have a type parameter implicitly specified:
    let _t = SGen(A); // Uses `A` defined at the top.
    let _i32 = SGen(6); // Uses `i32`.
    let _char = SGen('a'); // Uses `char`.

    // Using the non-generic functions
    reg_fn(S(A)); // Concrete type.
    gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}
