    mod checked {
#[derive(Debug)]
enum MathError{DivisionByZero,NonPositiveLogarithm,NegativeSquareRoot,}
type MathResult=Result<f64, MathError>;

fn div(x:f64,y:f64)->MathResult{if y==0.0{Err(MathError::DivisionByZero)}else{Ok(x/y)}}

fn sqrt(x:f64)->MathResult{if x<0.0{Err(MathError::NegativeSquareRoot)}else{Ok(x.sqrt())}}

fn ln(x:f64)->MathResult{if x<=0.0{Err(MathError::NonPositiveLogarithm)}else{Ok(x.ln())}}

fn op_(x:f64,y:f64)->MathResult{let ratio=div(x,y)?;let ln=ln(ratio)?;sqrt(ln)}

pub fn op(x:f64,y:f64){match op_(x,y){Ok(value)=>println!("{}",value),Err(why)=>panic!(match why{
    MathError::NonPositiveLogarithm=>"logarithm of non-positive number",
    MathError::DivisionByZero=>"division by zero",
    MathError::NegativeSquareRoot=>"square root of negative number",}),}}

    }

fn division(dividend:i32,divisor:i32)->i32{
    if divisor==0{panic!("division by zero");}else{dividend/divisor}}

    fn main() {
//checked::op(1.0, 10.0);
checked::op(1.0, 1.0);
// panic!
let _x=Box::new(0i32);
division(3,0);println!("This point won't be reached!");
    }
