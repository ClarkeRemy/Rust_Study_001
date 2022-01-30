use super::*;

#[test]
fn test_add() {assert_eq!(add(1, 2),3);}

#[test]
#[should_panic]
fn test_bad_add() {assert_eq!(bad_add(1,2),3);}

#[test]
fn test_sqrt()->Result<(),String>{let x = 4.0;assert_eq!(sqrt(x)?.powf(2.0),x);Ok(())}


#[test]
fn test_divide(){assert_eq!(divide_non_zero_result(10,2),5);}

#[test]
#[should_panic]
fn test_any_panic(){divide_non_zero_result(1,0);}

#[test]
#[should_panic(expected = "Divide result is zero")]
fn test_specific_panic(){divide_non_zero_result(1,10);}


#[test]
fn test_add_again() {assert_eq!(add(2,2),4);}
    
#[test]
fn test_add_hundred(){assert_eq!(add(100,2),102);assert_eq!(add(2,100),102);}

#[test]
#[ignore]
fn ignored_test(){assert_eq!(add(0,0),0);}
