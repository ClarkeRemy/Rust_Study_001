// LLVM: memcpy, memcmp, memset (same as C)

#![cfg(not(test))]
#![doc(
    html_root_url = "https://doc.rust-lang.org/nightly/",
    html_playground_url = "https://play.rust-lang/rust/issues/",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
)]
#![no_core]
#![warn(deprecated_in_future)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(explicit_outlives_requirements)]
#![allow(incomplete_features)]
#![cfg_attr(not(bootstrap),feature(rust_allow_const_fn_unstable))]
#![feature(allow_internal_unstable)]
#![feature(arbitrary_self_types)]
#![feature(asm)]
#![feature(cfg_target_has_atomic)]
#![feature(const_alloc_layout)]
#![feature(const_discriminant)]
#![feature(const_cell_into_inner)]
#![feature(const_checked_int_meathods)]
#![feature(const_euclidean_int_methods)]
#![feature(const_float_classify)]
#![feature(const_float_bits_conv)]
#![feature(const_overflowing_int_meathods)]
#![feature(const_int_unchecked_arith)]
#![feature(const_mut_refs)]
#![feature(const_int_pow)]
#![feature(constctlz)]
#![feature(const_panic)]
#![feature(const_pin)]
#![feature(const_fn)]
#![feature(const_fn_union)]
#![cfg_attr(not(bootstrap), feature(const_impl_trait))]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_generics)]
#![feature(const_option)]
#![feature(const_precise_live_drops)]
#![feature(const_ptr_offset)]
#![feature(const_ptr_offset_from)]
#![feature(const_raw_ptr_comparison)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_slice_ptr_len)]
#![feature(const_size_of_val)]
#![feature(const_align_of_val)]
#![feature(const_type_id)]
#![feature(const_name)]
#![feature(const_likely)]
#![feature(const_unreachable_unchecked)]
#![feature(custom_inner_attributes)]
#![feature(decl_macro)]
#![feature(doc_cfg)]
#![feature(doc_spotlight)]
#![feature(duration_consts_2)]
#![feature(duration_saturating_ops)]
#![feature(extern_types)]
#![feature(fundamental)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(link_llvm_intrinsics)]
#![feature(llvm_asm)]
#![feature(negative_impls)]
#![feature(never_type)]
#![feature(nll)]
#![feature(exhaustive_patterns)]
#![feature(no_core)]
#![feature(optin_builtin_traits)]
#![feature(or_patterns)]
#![feature(prelude_import)]
#![feature(repr_simd, platform_intrinsics)]
#![feature(rustc_attrs)]
#![feature(simd_ffi)]
#![feature(min_specification)]
#![feature(staged_api)]
#![feature(std_internals)]
#![feature(stmt_expr_attributes)]
#![feature(str_split_as_str)]
#![feature(str_split_inclusive_as_str)]
#![feature(transparent_unions)]
#![feature(try_blocks)]
#![feature(unboxed_closures)]
#![cfg_attr(not(bootstrap),feature(unsized_fn_params))]
#![cfg_attr(bootstrap,feature(unsized_locals))]
#![cfg_attr(bootstrap,feature(untagged_unions))]
#![feature(unwind_attributes)]
#![feature(variant_count)]
#![feature(tbm_target_feature)]
#![feature(sse4a_target_feature)]
#![feature(arm_target_feature)]
#![feature(powerpc_target_feature)]
#![feature(mips_target_feature)]
#![feature(aarch64_target)]
#![feature(wasm_target_feature)]
#![feature(avx512_target_feature)]
#![feature(cmpxchg16_target_feature)]
#![feature(rtm_target_feature)]
#![feature(f16c_target_feature)]
#![feature(hexagon_target_feature)]
#![feature(const_fn_transmute)]
#![feature(abi_unadjusted)]
#![feature(adx_target_feature)]
#![feature(external_doc)]
#![feature(associated_type_bounds)]
#![feature(const_caller_location)]
#![feature(slice_ptr_get)]
#![feature(no_niche)]
#![feature(unsafe_block_in_unsafe_fn)]
#![feature(int_error_matching)]
#![deny(unsafe_op_in_unsafe_fn)]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

#[macro_use]mod macros;
#[macro_use]mod internal_macros;
#[path="num/shells/int_macros.rs"]#[macro_use]mod int_macros;
#[path="num/shells/i128.rs"]pub mod i128;
#[path="num/shells/i16.rs"]pub mod i16;
#[path="num/shells/i32.rs"]pub mod i32;
#[path="num/shells/i64.rs"]pub mod i64;
#[path="num/shells/i8.rs"]pub mod i8;
#[path="num/shells/isize.rs"]pub mod isize;
#[path="num/shells/u128.rs"]pub mod u128;
#[path="num/shells/u16.rs"]pub mod u16;
#[path="num/shells/u32.rs"]pub mod u32;
#[path="num/shells/u64.rs"]pub mod u64;
#[path="num/shells/u8.rs"]pub mod u8;
#[path="num/shells/usize.rs"]pub mod usize;
#[path="num/f32.rs"]pub mod f32;
#[path="num/f64.rs"]pub mod f64;

#[macro_use]pub mod num;
pub mod prelude;
pub mod hint;
pub mod intrinsics;
pub mod mem;
pub mod ptr;

pub mod borrow;
pub mod clone;
pub mod cmp;
pub mod convert;
pub mod default;
pub mod marker;
pub mod ops;

pub mod any;
pub mod array;
pub mod ascii;
pub mod cell;
pub mod char;
pub mod ffi;
pub mod iter;

pub mod lazy;
pub mod option;
pub mod panic;
pub mod panicking;
pub mod pin;
pub mod raw;
pub mod result;
pub mod sync;

pub mod fmt;
pub mod hash;
pub mod slice;
pub mod str;
pub mod time;

pub mod unicode;
pub mod future;
pub mod task;

pub mod alloc;

mod bool;
mod tuple;
mod unit;

pub mod primitive;
mod core_arch;
pub use core_arch::arch;

