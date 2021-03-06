// List a = Empty | Elem a (List a)
use std::rc::Rc;

pub struct List<T>{head:Link<T>}
struct Node<T> {elem:T,next: Link<T>}
type Link<T> = Option<Rc<Node<T>>>;

impl<T> List<T>{
  pub fn new()-> Self {List { head: Link::None }}
  pub fn prepend(&self, elem: T)->List<T>{ List{ head: Some(Rc::new( Node{elem: elem, next: self.head.clone()}))} }
  pub fn tail(&self)->List<T> { List{ head:self.head.as_ref().and_then(|node| node.next.clone() )} }
  pub fn head(&self) -> Option<&T> {self.head.as_ref().map(|node| &node.elem)}
}

pub struct IntoIter<T>(List<T>);
impl<T> List<T> {pub fn into_iter(self)->IntoIter<T>{IntoIter(self)}}

pub struct Iter<'a,T> {next: Option<&'a Node<T>>}
impl<T> List<T> { pub fn iter(&self) -> Iter<T> {Iter{next: self.head.as_deref()}}}
impl<'a,T> Iterator for Iter<'a, T> {type Item = &'a T; 
  fn next(&mut self)-> Option<Self::Item> { self.next.map(|node| {self.next = node.next.as_deref();&node.elem})}}

impl<T> Drop for List<T> { fn drop(&mut self) {
  let mut head = self.head.take();
  while let Some(node) = head { if let Ok(mut node) = Rc::try_unwrap(node) {
    head = node.next.take();} else {break;}} }}

mod test;