#[cfg(test)]
mod tests;

pub fn add(a:i32,b:i32)->i32{a+b}

pub fn divide_non_zero_result(a: u32, b: u32)->u32{
if b==0{panic!("Divide-by-zero error");}else if a<b{panic!("Divide result is zero");}
a/b
}

#[allow(dead_code)]
fn bad_add(a: i32, b: i32)->i32{a-b}

fn sqrt(number: f64)->Result<f64,String>{
if number>=0.0{Ok(number.powf(0.5))
}else{Err("negative floats don't have square roots".to_owned())}
}

fn main() {
    println!("Hello, world!");
}
