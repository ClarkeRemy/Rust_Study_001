# __Operators and Symbols__

### __Operators__

Table B-1 contains the operators in Rust,
an example of how the operator would appear in context,
a short explanation,
and whether that operator is overloadable.
If an operator is overloadable,
the relevant trait to use to overload that operatoris listed.

__Operator__|__Example__|__Explanation__|__Overloadable?__
-|-|-|-
`!  `|`ident!(...)`, `ident!{...}`, `ident![...]`|Macroexpansion
`!  `|`!expr`|Bitwise or logical complement|`Not`
`!= `|`var != expr`|Nonequality comparison|`PartialEq`
`%  `|`expr % expr`|Arithmetic remainder|`Rem`
`%= `|`var %= expr`|Arithmetic remainder and assignment|`RemAssign`
`&  `|`&expr`, `&mut expr`|Borrow
`&  `|`&type`, `&mut type`, `&'a type`, `&'a mut type`|Borrowed pointer type
`&  `|`expr & expr`|Bitwise AND|`BitAnd`
`&= `|`var &= expr`|Bitwise AND and assignment|`BitAndAssign`
`&& `|`expr && expr`|Short-circuiting logical AND
`*  `|`expr * expr`|Arithmetic multiplication|`Mul`
`*= `|`var *= expr`|Arithmetic multiplication and assignment|`MulAssign`
`*  `|`*const type`,`*mut type`|Raw pointer
`+  `|`trait + trait`, `'a + trait `|Compound type constraint
`+  `|`expr + expr`|Arithmetic addition|`Add`
`+= `|`var += expr`|Arithmetic addition and assignment|`AddAssign`
`,  `|`expr, expr`|Argument and element separator
`-  `|`expr - expr`|Arithmetic subtraction|`Sub`
`-= `|`var -= expr`|Arithmetic subtraction and assignment|`SubAssign`
`-> `|`fn(...) -> type`, `|...| -> type`|Function and closure return type
`.  `|`expr.ident`|Member access
`.. `|`..`, `expr..`, `expr..expr`|Right-exclusive range
`.. `|`variant(x, ..)`, `struct_type { x, ..}`|"And the rest" pattern biding
`...`|`expr...expr`|In a patern: inclusive range patern
`/  `|`expr / expr`|Arithmetic division |`Div`
`/= `|`var /= expr`|Aritmetic division and assignment|`DivAssign`
`:  `|`pat: type`, `ident: type`|Constraints
`:  `|`ident: expr`|Struct field initializer
`:  `|`'a; loop{...}`|Loop label
`;  `|`expr;`|Statement and item terminator
`;  `|`[...; len]`|Part of fixed-size array syntax
`<< `|`expr << expr`|Left-shift|`Shl`
`<<=`|`var <<= expr`|Left-shift and assignment|`ShlAssign`
`<  `|`expr < expr`|Less than comparison|`PartiolOrd`
`=  `|`var = expr`, `ident = type`|Assignment/equivalence
`== `|`expr == expr`|Equallity Comparison|`PartialEq`
`=> `|`pat => expr`|part of match arm syntax
`>  `|`expr > expr`|Greater than comparison|`PartialOrd`
`>= `|`expr >= expr`|Greater than or equal to comparison|`PartialOrd`
`>> `|`expr >> expr`|Right-shift|`SHa`
`>>=`|`var >>= expr`|Righr-shift and assignment|`ShrAssign`
`@  `|`ident @ pat`|Pattern Binding
`^  `|`expr ^ expr`|Bitwise exclusive OR|`BitXor`
`^= `|`expr ^= expr`|Bitwise exclusive OR and assignment|`BitXorAssign`
`|  `|`pat | pat `|Pattern alternatives
`|  `|`expr | expr`|Bitwise OR|`BitOr`
`|= `|`var |= expr`|Bitwise OR and assignment|`BitOrAssign`
`|| `|`expr || expr`|Short-cicuiting logical OR
`?  `|`expr?`|Error propagation

***

### __Non-operator Symbols__

the following list contains all non-letters that don't function as operators; that is, they don't behave like a function or method call.

Table B-2 shows symbols that appear on their own and are valid in a variety of locations.

Table B-2: Stand-Alone Syntax

__Symbol__|__Explanation__
-|-
``` `ident ```|Named lifetime or loop lable
`...u8`, `...i32`, `...f64`, `...usize`, etc.|Numeric literal of specific type
`"..."`|String literal
`r"..."`, `r#"..."#`, `r##"..."##`, etc.|Raw string literal, escape characters not processed
`b"...`|Byte string litteral; constructs a [u8] instead of a string
`br"..."`, `br#"...#"`, `br##"...##"`, etc. |Raw byte string literal, combination of raw and byte string literal
`'...'`|Character literal
`b'..'`| ASCII byte literal
`|...| expr`|closure
`!`|Always empty bottom type for diverging functions
`_`|"ignored" pattern binding; also used to make integer literals readable

Table B-3 shows symbols that appear in the context of a path through the module hierarchy to an item.

Table b-3: Path-Related Syntax

__Symbol__|__Explanation__
-|-
`ident::ident`|Namespace path
`::path`|Path relative to the crate root (i.e., an explicitly absolute path)
`self::path`|Path relative to the current module (i.e., an explicitly relative path).
`super::path`|Path relative to the parent of the current module
`type::ident`, `<type as trait>::ident`|Associated constants,functions, and types
`<type>::...`|Associated item for a type that cannot be directly named (e.g., `<&T>::...`, `<[T]>::...`, etc.)
`trait::method(...)`|Disambiguating amethod call by naming the trait that defines it
`type::method(...)`|Disambiguating amethod call by naming the type for which it's defined
`<type as trait>::method(...)`|Disambiguating amethod call by naming the trait and type

Table B-4 showa symbols that appear in the context of using generic type parameters.

Table B-4: Generics

__Symbol__|__Explanation__
-|-
`path<...>`|Specifies parameters to generic tyoe in a type (e.g., `Vec<u8>`)
`path::<...>`, `method::<...>`|Specifies parameters to generic type, function, or method in an expression; often referred to as turbofish(e.g., `"42".parse::<i32>()`)
`fn ident<...> ...`|Define generic function
`struct ident<...> ...`|Define generic structure
`enum ident<...> ...`|Define generic enumeration
`for<...> type`|Higher-ranked lifetime bounds
`type<ident=type>`|A generic type where one or more associated types have assignments (e.g., `Iterator<Item=T>`)

Table B-5 shows symbols that appear in the context of constraining generic type parameters with trait bounds.

Table B-5: Trait Bound Constraints

__Symbol__|__Explanation__
-|-
`T: U`|Generic parameter `T` constrained to types that implement `U`
`T: 'a`|Generic parameter `T` must outlive lifetime `'a` (meaning the type cannot transitively contain any references with lifetimes shorter than `'a`)
`T: 'static`|Generic type `T` contains no borrowed references other than `'static` ones
`'b: 'a`|Generic lifetime 'b must outlive lifetime 'a
`T: ?Sized`|Allow generic type parameter to be a dynamically sized type
`'a + trait`, `trait + trait`|Compound type constraint

Table B-6 symbols that appear in the context of calling or defining macros and specifying attributes on an item.

Table B-6: Macros and Attributes

__Symbol__|__Explanation__
-|-
`#[meta]`|Outer attribute
`#![meta]`|Inner attribute
`$ident`|Macro substitution
`$ident:kind`|Macro capture
`$(...)...`|Macro Repetition
`ident!(...)`, `ident!{...}`, `ident![...]`|Macro invocation

Table B-7 shows symbols that create comments.

Table B-7: Comments

__Symbol__|__Explanation__
-|-
`//`|Line comment
`//!`|Inner line doc comment
`///`|Outer line doc comment
`/*...*/`|Block comment
`/*!...*/`|Inner block comment
`/**...*/`|Outer block comment

Table B-8 shows symbols that appear in the contextof using tuples.

Table B-8: Tuples

__Symbol__|__Explanation__
-|-
`()`|Empty tuple (aka unit), both literal and type
`(expr)`|Parenthesized expression
`(expr,)`|Single-element tuple expresion
`(expr, ...)`|Tuple expression
`(type,...)`|Tuple type
`expr(expr, ...)`|Function call expression; also used to initialize tuple `struct`s and tuple `enum` variants
`expr.0`, `expr.1`, etc.|Tuple indexing

Table B-9 shows the contents in which curly braces are used.

Table B-9: CurlyBrackets

__Symbol__|__Explanation__
-|-
`{...}`|Block expression
`Type {...}`|`struct` literal

Table B-10 shows the contents in which square brackets are used.

Table B-10: Square Brackets

__Symbol__|__Explanation__
-|-
`[...]`|Array literal
`[expr; len]`|Array literal containing `len` copies of `expr`
`[type; len]`|Array type containing len instances of `type`
`expr[expr]`|Collection indexing. Overloadable (`Index`, `IndexMut`)
`expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]`, |Collection indexing pretending to be collection slicing, using `Range`, `RangeTo`, or `RangeFull` as the "Index"