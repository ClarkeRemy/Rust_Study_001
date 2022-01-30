fn main() {
    //factorial
    let f = |a:u128| {if a==0{ return 1}; (1..=a).product::<u128>()};
    let res = |y|{
        let fy = f(y) ;
        (0_u128..=y).map(| x | 
        // combinations * paths   
        fy/( f(x) * f(y-x) )*x  )
        .sum::<u128>()};
    
    let (min,max) = (0,30);
    // print the ammounts
    println!();
    for i in min..=max{
       println!("{} :: {}",i, res(i) );
    }
}

// (+/ % #)@:(2&%/\)@:(+/@:* (!/ {:))@:i."0  >:i. 30 
// load 'plot'
//    plot (+/ % #)@:(2&(%/\))@:((+/@:* (!/ {:))@:i.@:>:"0)@:(i."0) i.100
//
// *./@:(((*  (2^ -&1))=  (+/@:*  (!/  {:))@:i.@:>: )"0) i.100