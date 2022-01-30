#[allow(unused_imports)]use super::List;
#[test] fn basics(){
  let mut list = List::new();
  
  // Check empty list behaves right
  assert_eq!(list.pop(), None);

  // Populate list
  list.push(1);
  list.push(2);
  list.push(3);

  // Check normal removal
  assert_eq!(list.pop(),Some(3));
  assert_eq!(list.pop(), Some(2));

  // Push some more just to make sure nothing's corrupted
  list.push(4);
  list.push(5);

  // Check normal removal
  assert_eq!(list.pop(), Some(5));
  assert_eq!(list.pop(), Some(4));

  // Check exhaustion
  assert_eq!(list.pop(), Some(1));
  assert_eq!(list.pop(), None);
}

