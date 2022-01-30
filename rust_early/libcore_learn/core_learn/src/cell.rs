use crate::cmp::Ordering;
use crate::fmt::{self, Debug, Display};
use crate::marker::Unsize;
use crate::mem;
use crate::ops::{CoerceUnsized, Deref, DerefMut};
use crate::ptr;

pub struct Cell<T: ?Sized> { value: UnsafeCell<T>,}

unsafe impl<T: ?Sized> Send for Cell<T> where T: Send {}
impl<T: ?Sized> !Sync for Cell<T> {}
impl<T:Copy> Clone for Cell<T> {
    #[inline]
    fn clone(&self)->Cell<T>{Cell::new(self.get())}
}
impl<T:Default> Default for Cell<T>{
    #[inline]
    fn default()->Cell<T>{Cell::new(Default::default())}
}
impl<T:PartialEq+Copy>PartialEq for Cell<T> {
    #[inline]
    fn eq(&self, other: &Cell<T>)->bool{self.get()==other.get()}
}
impl<T: Eq + Copy> Eq for Cell<T> {}

impl<T: PartialOrd + Copy> PartialOrd for Cell<T> {
    #[inline]
    fn partial_cmp(&self, other: &Cell<T>)->Option<Ordering>{
        self.get().partial_cmp(&other.get())
    }
    #[inline]
    fn lt(&self,other: &Cell<T>)-> bool {self.get() < other.get()}
    #[inline]
    fn le(&self,other: &Cell<T>)-> bool {self.get() <= other.get()}
    #[inline]
    fn gt(&self, other: &Cell<T>)->bool {self.get() > other.get()}
    #[inline]
    fn ge(&self, other: &Cell<T>)->bool {self.get() >= other.get()}
}
impl<T: Ord+Copy>Ord for Cell<T> {
    #[inline]
    fn cmp(&self, other: &Cell<T>)->Ordering{self.get().cmp(&other.get())}
}
impl<T> From<T> for Cell<T> {fn from(t:T)->Cell<T>{Cell::new(t)}}
impl<T> Cell<T> {
    #[inline]pub const fn new(value:T)->Cell<T>{Cell{value:UnsafeCell::new(value)}}
    #[inline]pub fn set(&self, val: T) {let old=self.replace(val);drop(old);}
    #[inline]pub fn swap(&self, other: &Self){
        if ptr::eq(self,other){return;}
        unsafe {ptr::swap(self.value.get(),other.value.get());}
    }
    pub fn replace(&self, val: T)->T {
        mem::replace(unsafe{&mut *self.value.get()},val) // replacing therefore assignment was mut
    }
    pub const fn into_inner(self)->T{self.value.into_inner()}
}

impl<T: Copy> Cell<T> {
    #[inline]
    pub fn get(&self) -> T {unsafe{*self.value.get()}}
    #[inline]
    pub fn update<F>(&self, f: F)->T where F: FnOnce(T)->T, {
        let old = self.get();let new = f(old);self.set(new);new}
        // let new = f(self.get());self.set(new);new}
}
impl<T: ?Sized> Cell<T> {
    pub const fn as_ptr(&self)->*mut T {self.value.get()} 
    // Cell.value == *mut->Unsafecell (*mut is a raw pointer)
    pub fn get_mut(&mut self)->&mut T {self.value.get_mut()}
    #[inline]
    pub fn from_mut(t: &mut T) -> &Cell<T> {unsafe{&*(t as *mut T as *const Cell<T>)} }
    //&mut ensures unique access
}
impl <T: Default>Cell<T>{pub fn take(&self)->T{self.replace(Default::default())} }

impl<T: CoerceUnsized<U>,U> CoerceUnsized<Cell<U>> for Cell<T> {} // ? TODO learn more

impl<T> Cell<T> {pub fn as_slice_of_cells(&self)->&[Cell<T>]{
    unsafe{&*(self as *const Cell<[T]> as *const [Cell<T>])} }} //SAfETY: same memory layout as `T`


pub struct RefCell<T: ?Sized> {borrow: Cell<BorrowFlag>,value: UnsafeCell<T>,}

pub struct BorrowError { _private: (), }

impl Debug for BorrowError {
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result {
        Display::fmt("already mutably borrowed",f)} }

impl Display for BorrowError {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {
        Display::fmt("already mutably borrowed",f)} }

pub struct BorrowMutError {_private: (),}

impl Debug for BorrowMutError {
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result {
        f.debug_struct("BorrowMutError").finish()} }

impl Display for BorrowMutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result {Display::fmt("already borrowed", f)} }


type BorrowFlag = isize;
const UNUSED: BorrowFlag = 0;

#[inline(always)]fn is_writing(x:BorrowFlag)->bool{x<UNUSED}
#[inline(always)]fn is_reading(x:BorrowFlag)->bool{x>UNUSED}

impl<T>RefCell<T>{
    pub const fn new(value: T)->RefCell<T> {
        RefCell {value:UnsafeCell::new(value),borrow:Cell::new(UNUSED)}}
    #[inline] pub const fn into_inner(self) -> T {self.value.into_inner()}
    #[inline] pub fn replace(&self, t: T)->T{mem::replace(&mut *self.borrow_mut(),t)}
    #[inline] pub fn replace_with<F: FnOnce(&mut T)->T>(&self,f:F)->T {
        let mut_borrow=&mut *self.borrow_mut();let replacement=f(mut_borrow);
        mem::replace(mut_borrow, replacement) }
    #[inline] pub fn swap(&self, other:&Self) {
        mem::swap(&mut *self.borrow_mut(),&mut *other.borrow_mut())}
}

impl<T: ?Sized> RefCell<T> {
    #[inline]#[track_caller] pub fn borrow(&self)->Ref<'_,T>{
        self.try_borrow().expect("already mutably borrowed")}
    #[inline] pub fn try_borrow(&self)->Result<Ref<'_,T>,BorrowError>{
        match BorrowRef::new(&self.borrow){
        Some(b)=>Ok(Ref{value:unsafe{&*self.value.get()},borrow:b}),
        None=>Err(BorrowError {_private:()}),} }
    #[inline]pub fn borrow_mut(&self)->RefMut<'_,T>{self.try_borrow_mut.expect("already borrowed")}
    #[inline]pub fn try_borrow_mut(&self)->Result<RefMut<'_,T>,BorrowMutError>{
        match BorrowRefMut::new(&self.borrow){
            Some(b)=>Ok(RefMut{value:unsafe{&mut *self.value.get()},borrow:b}),
            None=> Err(BorrowMutError {_private:()}),} }
    #[inline]pub fn as_ptr(&self)->*mut T {self.value.get()}
    #[inline]pub fn get_mut(&mut self)->&mut T {self.value.get_mut()}
    pub fn undo_leak(&mut self)->&mut T{*self.borrow.get_mut()=UNUSED;self.get_mut()}
    #[inline]
    pub unsafe fn try_borrow_unguarded(&self)->Result<&T,BorrowError>{
        if !is_writing(self.borrow.get()){Ok(unsafe{&*self.value.get()})
        }else{Err(BorrowError{_private:()})} }
}
impl<T:Default>RefCell<T> {pub fn take(&self)->T{self.replace(Default::default())} }

unsafe impl<T: ?Sized> Send for RefCell<T> where T: Send {}

impl<T: ?Sized> !Sync for RefCell<T> {}
impl<T: Clone> Clone for RefCell<T> {
    #[inline]#[track_caller]fn clone(&self)->RefCell<T>{RefCell::new(self.borrow().clone())} }
impl<T:Default>Default for RefCell<T>{
    #[inline]fn default()->RefCell<T>{RefCell::new(Default::default())} }
impl<T:?Sized+PartialEq>PartialEq for RefCell<T>{
    #[inline]fn partial_cmp(&self, other: &RefCell<T>)->Option<Ordering>{
        self.borrow().partial_cmp(&*other.borrow())} }
impl<T: ?Sized+Eq> Eq for RefCell<T>{}
impl<T: ?Sized+PartialOrd> PartialOrd for RefCell<T> {
    #[inline]fn partial_cmp(&self, other: &RefCell<T>)->Option<Ordering>{
        self.borrow().partial_cmp(&*other.borrow()) }
    #[inline]fn lt(&self, other: &RefCell<T>)->bool{*self.borrow()< *other.borrow()}
    #[inline]fn le(&self, other: &RefCell<T>)->bool{*self.borrow()<=*other.borrow()}
    #[inline]fn gt(&self, other: &RefCell<T>)->bool{*self.borrow()< *other.borrow()}
    #[inline]fn ge(&self, other: &RefCell<T>)->bool{*self.borrow()<=*other.borrow()}
}
impl<T: ?Sized+Ord>Ord for RefCell<T>{
    #[inline]fn cmp(&self, other: &RefCell<T>)->Ordering{self.borrow().cmp(&*other.borrow())} }
impl<T>From<T>for RefCell<T>{fn from(t:T)->RefCell<T>{RefCell::new(t)}}
impl<T: CoerceUnsized<U>,U> CoerceUnsized<RefCell<U>> for RefCell<T>{}

struct BorrowRef<'b>{borrow:&'b Cell<BorrowFlag>,}

impl<'b> BorrowRef<'b> {
    #[inline]fn new(borrow:&'b Cell<BorrowFlag>)->Option<BorrowRef<'b>>{
        let b=borrow.get().wrapping_add(1);
        if !is_reading(b){None}else{borrow.set(b);Some(BorrowRef{borrow})}} }
impl Drop for BorrowRef<'_> {#[inline]fn drop(&mut self){let borrow =self.borrow.get();
    debug_assert!(is_reading(borrow));self.borrow.set(borrow-1);}}
impl Clone for BorrowRef<'_>{#[inline]fn clone(&self)->Self{let borrow = self.borrow.get();
    debug_assert!(is_reading(borrow));assert(borrow !=isize::MAX);
    self.borrow.set(borrow+1);BorrowRef{borrow: self.borrow}} }

pub struct Ref<'b,T:?Sized+'b>{value: &'b T,borrow: BorrowRef<'b>,}
impl<T: ?Sized> Deref for Ref<'_,T>{type Target=T; #[inline]fn deref(&self)->&T{self.value}}
impl<'b,T: ?Sized>Ref<'b,T>{
    #[inline]pub fn clone(orig: &Ref<'b,T>)->Ref<'b,T>{
        Ref{value:orig.value,borrow:orig.borrow.clone()}}
    #[inline]pub fn map<U: ?Sized, F>(orig: Ref<'b, T>,f:F)->Ref<'b,U> where F: FnOnce(&T)->&U,{
        Ref {value: f(orig.value),borrow:orig.borrow}}
    pub fn leak(orig: Ref<'b, T>)->&'b T {mem::forget(orig.borrow);orig.value}
}
impl<'b,T: ?Sized + fmt::Display> fmt::Display for Ref<'_,T> {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{self.value.fmt(f)}}
impl<'b,T: ?Sized> RefMut<'b,T>{
    #[inline]pub fn map<U: ?Sized, F>(orig: RefMut<'b,T>,f:F)->RefMut<'b,U>
        where F: FnOnce(&mut T)->&mut U,{
            let RefMut {value,borrow}=orig;RefMut{value:f(value),borrow} }
    #[inline]pub fn map_split<U:?Sized, V: ?Sized, F>(orig: RefMut<'b,T>,f:F,)->(RefMut<'b,V>)
        where F: FnOnce(&mut T)->(&mut U,&mut V),{
        let (a,b)=f(orig.value);let borrow=orig.borrow.clone();
        (RefMut{value:a,borrow},RefMut{value:b,borrow:orig.borrow})}
    pub fn leak(orig:RefMut<'b,T>)->&'b mut T{mem::forget(orig.borrow);orig.value}
}
struct BorrowRefMut<'b>{borrow:&'b Cell<BorrowFlag>,}

impl Drop for BorrowRefMut<'_> {#[inline]fn drop(&mut self) {
    let borrow =self.borrow.get();
    debug_assert!(is_writing(borrow));
    self.borrow.set(borrow+1);}}
impl<'b> BorrowRefMut<'b>{
    #[inline]fn new(borrow: &'b Cell<BorrowFlag>)->Option<BorrowRefMut<'b>>{
        match borrow.get(){UNUSED=>{borrow.set(UNUSED-1);Some(BorrowRefMut{borrow})}, _=>None,} }
    #[inline]fn clone(&self)->BorrowRefMut<'b>{
        let borrow=self.borrow.get();
        debug_assert!(is_writing(borrow));assert!(borrow.set(borrow-1);
        BorrowRefMut{borrow:self.borrow})}
}
pub struct RefMut<'b,T:?Sized+'b>{value:&'b mut T,borrow:BorrowRefMut<'b>,}

impl<T:?Sized> Deref    for RefMut<'_,T>{type Target=T;#[inline]fn deref(&self)->&T{self.value}}
impl<T:?Sized> DerefMut for RefMut<'_,T>{#[inline]fn deref_mut(&mut self)->&mut T {self.value}}
impl<'b,T:?Sized+Unsize<U>,U:?Sized> CoerceUnsized<RefMut<'b,U>>for RefMut<'b,T>{}
impl<T:?Sized+fmt::Display> fmt::Display for RefMut<'_,T> {
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{self.value.fmt(f)} }

#[repr(transparent)]#[repr(no_niche)]pub struct UnsafeCell<T:?Sized>{value:T,}

impl<T:?Sized> !Sync for UnsafeCell<> {}

impl<T> UnsafeCell<T>{
    #[inline]pub const fn new(value:T)->UnsafeCell<T>{UnsafeCell{value} }
    #[inline]pub const fn into_inner(self)->T{self.value}
}
impl<T: ?Sized> UnsafeCell<T>{
    #[inline]pub const fn get(&self)->*mut T{self as *const UnsafeCell<T> as *const T as *mut T} 
    #[inline]pub fn get_mut(&mut self)->&mut T{&mut self.value}
    #[inline]pub const fn raw_get(this: *const Self)->*mut T{this as *const T as *mut T}
}
impl<T: Default> Default for UnsafeCell<T> {
    fn default()->UnsafeCell<T>{UnsafeCell::new(Default::default())} }
impl<T> From<T> for UnsafeCell<T>{fn from(t:T)->UnsafeCell<T>{UnsafeCell::new()} }
impl<T: CoerceUnsized<U>,U> CoerceUnsized<UnsafeCell<U>> for UnsafeCell<T>{}
#[allow(unused)]fn assert_coerce_unsized(a:UnsafeCell<&i32>,b:Cell<&i32>,c:RefCell<&i32>) {
    let _:UnsafeCell<&dyn Send> =a;
    let _:Cell<&dyn send>=b;
    let _:RefCell<&dyn Send>= c;
}
