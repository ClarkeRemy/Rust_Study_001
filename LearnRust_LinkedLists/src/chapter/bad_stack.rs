// List a = Empty | Elem a (List a)


#[derive(Clone)]pub struct List{head:Link}
#[derive(Clone)]struct Node {elem:i32,next: Link}
#[derive(Clone)]enum Link{Empty, More(Box<Node>)}

impl List{
  pub fn new()-> Self {List { head: Link::Empty }}
  pub fn push(&mut self, elem: i32) {
    let new_node= Box::new(Node{elem:elem, next:std::mem::replace(&mut self.head,Link::Empty)});
    self.head= Link::More(new_node);}
  pub fn pop(&mut self)-> Option<i32>{
    match std::mem::replace(&mut self.head, Link::Empty) {
      Link::Empty=>None, 
      Link::More(node)=>{self.head=node.next;Some(node.elem)}} }
}
impl Drop for List {
  fn drop(&mut self) {
    let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
    while let Link::More(mut boxed_node) = cur_link { cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);} }}

mod test;