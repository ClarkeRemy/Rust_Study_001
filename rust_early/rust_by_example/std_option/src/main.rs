    mod checked {
#[derive(Debug)]
pub enum MathError{DivisionByZero,NonPositiveLogarithm,NegativeSquareroot,}
pub type MathResult=Result<f64,MathError>;

pub fn div(x:f64,y:f64)->MathResult{if y == 0.0{Err(MathError::DivisionByZero)}else{Ok(x/y)}}
pub fn sqrt(x:f64)->MathResult{if x<0.0{Err(MathError::NegativeSquareroot)}else{Ok(x.sqrt())}}
pub fn ln(x:f64)->MathResult{if x<=0.0{Err(MathError::NonPositiveLogarithm)}else{Ok(x.ln())}}
    }

fn op(x:f64,y:f64)->f64{match checked::div(x,y){
    Err(why)=>panic!("{:?}",why),Ok(ratio)=>match checked::ln(ratio){
        Err(why)=>panic!("{:?}",why),Ok(ln)=>match checked::sqrt(ln){
            Err(why)=>panic!("{:?}",why),Ok(sqrt)=>sqrt,}}}}

fn checked_division(dividend:i32,divisor:i32)->Option<i32>{
    if divisor==0{None}else{Some(dividend/divisor)}}

fn try_division(dividend:i32,divisor:i32){match checked_division(dividend, divisor){
    None=>println!("{} / {} failed!",dividend,divisor),
    Some(quotient)=>println!("{} / {} = {}",dividend,divisor,quotient),}}

    fn main() {
try_division(4,2);try_division(1, 0);
let none:Option<i32>=None;let _equivalent_none=None::<i32>;
let optional_float=Some(0f32);

println!("{:?} unwraps to {:?}",optional_float,optional_float.unwrap());
//println!("{:?} unwraps to {:?}",none,none.unwrap());

// Result

println!("{}", op(1.0, 10.0));
    }
