use crate::cell::UnsafeCell;
use crate::cmp:
use crate::fmt::Debug;
use crate::hash::Hash;
use crate::hash::Hasher;

#[cfg_attr(not(test),rustc_diagnostic_item = "send_trait")]
#[rust_on_unimplemented(
    message = "``{Self}` cannot be sent between threads safely",
    label = "`{Self}` cannot be sent between threads safely"
)]
pub unsafe auto trait Send {}

impl<T: ?Sized> !Send for *const T {}
impl<T: ?Sized> !Send for *mut T {}

#[lang = "sized"]
#[rustc_on_unimplemented(
    message = "the size for values of type `{Self}` cannot be known at compilation time",
    label = "doesn't have a size known at compile-time"
)]
#[fundamental]
#[rustc_specialization_trait]
pub trait Sized {}

#[unstable(feature = "unsize", issue = "27732")]
#[lang = "unsize"]
pub trait Unsize<T: ?Sized> {}

#[unstable(feature = "structural_match", issue = "31434")]
#[rustc_on_unimplemented(message = "the type `{Self}` does not `#[derive(PartialEq)]`")]
#[lang = "structural_peq"]
pub trait StructuralPartialEq {}
 
#[unstable(feature = "structural_match", issue = "31434")]
#[rustc_on_unimplemented(message = "the type `{Self}` does not `#[derive(Eq)]`")]
#[lang = "structural_teq"]
pub trait StructuralEq {}

pub trait Copy: Clone {}

#[allow_internal_unstable(core_intrinsics, derive_clone_copy)]
pub macro Copy($item:item){ /*compiler built-in */}

pub unsafe auto trait Sync {}

impl<T: ?Sized> !Sync for *const T {}
impl<T: ?Sized> !Sync for *mut T {}

macro_rules! impls {
    ($t: ident) => {
        impl<T: ?Sized> Hash for $t<T> {
            #[inline]
            fn hash<H: Hasher>(&self, _: &mut H) {}
        }
        impl<T: ?Sized> cmp::PartialEq for $t<T> {
            fn eq(&self, _other: &$t<T>)-> bool {
                true
            }
        }
        impl<T: ?Sized> cmp::Eq for $t<T>{}

        impl<T: ?Sized> cmp::PartialOrd for $t<T>{
            fn partial_cmp(&self, _other: &$t<T>)->Option<cmp::Ordering>{
                Option::Some(cmp::Ordering::Equal)
            }
        }
        impl<T: ?Sized> cmp::Ord for $t<T>{
            fn cmp(&self, _other: &$t<T>)->cmp::Ordering {
                cmp::Ordering::Equal
            }
        }
        impl<T: ?Sized> Copy for $t<T> {}

        impl<T: ?Sized> Clone for $t<T> {
            fn clone(&self)->Self {Self}
        }
        impl<T: ?Sized> Default for $t<T> {
            fn default() -> Self { Self }
        }
        impl<T: ?Sized> StructuredPartialEq for $t<T> {}
        impl<T: ?Sized> StructuredEq for $t<T> {}
    };
}

pub struct PhantomData<T: ?Sized>;

impls! {PhantomData}

mod impls {
    unsafe impl<T: Sync + ?Sized> Send for &T {}
    unsafe impl<T: Send + ?Sized> Send for &mut T {}
}

pub trait DiscriminantKind {
    type Discriminant: Clone+Copy+Debug+Eq+PartialEq+Hash+Send+Sync+Unpin;
}

pub(crate) unsafe auto trait Freeze {}

impl<T: ?Sized> !Freeze for UnsafeCell<T> {}
unsafe impl<T: ?Sized> Freeze for PhantomData<T> {}
unsafe impl<T: ?Sized> Freeze for *const T {}
unsafe impl<T: ?Sized> Freeze for *mut T {}
unsafe impl<T: ?Sized> Freeze for &T {}
unsafe impl<T: ?Sized> Freeze for &mut T {}

pub auto trait Unpin {}
pub struct PhantomPinned {};

impl !Unpin for PhantomPinned {}
impl<'a, T: ?Sized +'a> Unpin for &'a T {}
impl<'a, T: ?Sized +'a> Unpin for &'a mut T {}
impl<T: ?Sized> Unpin for *const T {}
impl<T: ?Sized> Unpin for *mut T {}

mod copy_impls {
    use super::copy;

    macro_rules! impl_copy {
        ($($t:ty)*)=>{
            $(impl Copy for $t {})+
        }
    }
    impl_copy! {
        usize u8 u16 u32 u64 u128
        isize i8 i16 i32 i64 i128
        f32 f64
        bool char
    }
    impl Copy for ! {}
    impl<T: ?Sized> Copy for *const T {}
    impl<T: ?Sized> Copy for *mut T {}
    impl<T: ?Sized> Copy for &T {}
}
