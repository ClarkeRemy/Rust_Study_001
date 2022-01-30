fn main() {
    let penguin_data ="\
    common name,length (cm)
    Little penguin,33
    Yellow-eye penguin,65
    Fiordland penguin,60
    Invalid, data
    ";

    let records = penguin_data.lines();
    for (i, r) in records.enumerate() {
        match (i==0||r.trim().len()==0) {true=>continue, _=>() };
        let f: Vec<_> = r.split(',').map(|x|x.trim()).collect();
        match cfg!(debug_assertions) {
            true =>eprintln!( "debug: {:?} -> {:?}", r,f), _=>() };
        let n = f[0];
        let l = match f[1].parse::<f32>() { Err(_e)=>continue, Ok(l)=>l };
        println!("{}, {}cm", n, l);
    }
}
