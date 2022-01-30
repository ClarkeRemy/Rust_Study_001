use std::thread;

const NTHREADS: u32=10;

fn main() {
let mut children = vec![];
for i in 0..NTHREADS{children.push(thread::spawn(move||{
    {println!("this is thread number {}",i);}}));}
for child in children{let _=child.join();}

// Testcase mapreduce

let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";
let mut children =vec![];
let chunked_data=data.split_whitespace();
for (i, data_segment) in chunked_data.enumerate() {
    println!("data segment {} is \"{}\"",i,data_segment);
    children.push(thread::spawn(move||{
        let result=data_segment.chars().map(|c|c.to_digit(10).expect("should be a digit")).sum();
    println!("processed segment {}, result={}",i,result);})); }
let mut intermediate_sums=vec![];
for child in children{
    let intermediate_sum=child.join().unwrap();intermediate_sums.push(intermediate_sum);}
let final_result=intermediate_sums.into_iter().sum::<u32>();
println!("Final sum result: {}",final_result);
}
