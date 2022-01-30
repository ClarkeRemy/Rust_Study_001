use std::ops::RangeInclusive;

use itertools::Itertools;

fn main() {
	let (x, y) = (vec![3, 5, 7], vec![1, 2, 3]);
	let x_plus_y = x.iter().zip(y.iter()).map(|(x, y)| x + y).collect::<Vec<_>>();
	p(x_plus_y);

	p({ 1..=5 }.collect::<Vec<_>>());
	p({ 1..=7 }.step_by(2).collect::<Vec<_>>());
	p({ 1..=7 }.filter(|x| x % 2 == 1).map(|x| x + 3).collect::<Vec<_>>());
	p({ 1..=5 }.map(|x| x * 3 + 2).collect::<Vec<_>>());
	p({ 1..=6 }.map(|x| x * 2 + 1).sum::<i32>());
	p({ 1..=6 }.map(|x| x * 2 + 1).fold(0, |x, y| x + y));
	p({ 1..=6 }.map(|x| x * 2 + 1).sum::<i32>() + 1);
	p(vec![2; 3]);
	p(vec![2; 3].iter().sum::<i32>());
	p(std::cmp::max(2, 3));
	p({ 1..=4 }.map(|x| std::cmp::max(2, x)).collect::<Vec<_>>());
	println!("{:?}", { 1..=5 }.max().unwrap());
	println!(
	         "{:?}",
	         vec![8, 1, 7, 10, 3, 10].iter().fold(0, |x, y| std::cmp::max(x, *y))
	);
	println!(
	         "{:?}",
	         { 1..=5 }.zip({ 1..=5 }.rev()).map(|(x, y)| std::cmp::max(x, y)).collect::<Vec<_>>()
	);
	let a = { 1..=8 }.collect::<Vec<_>>();
	let b = { 1..=10 }.collect::<Vec<_>>();
	// let o_p = a.iter().cartesian_product(b.iter()).collect::<Vec<_>>();
	let o_p = a.iter().map(|x| b.iter().map(|y| x * y).collect::<Vec<_>>()).collect::<Vec<_>>();
	println!("{:?}", V2D(o_p));

	let a = vec![1, 3, 5];
	let b = vec![2, 4, 6];
	println!(
	         "{:?}",
	         a.iter().zip(b.iter()).map(|(x, y)| x * y).collect::<Vec<_>>()
	);
	println!(
	         "{:?}",
	         V2D(a.iter()
	              .map(|x| b.iter().map(|y| y * x).collect::<Vec<_>>())
	              .collect::<Vec<_>>())
	);

	let b = { 1..=4 }.map(|x| x * 3)
	                 .map(|x| { 1..=6 }.map(|p| p * 2).map(|y| x + y + 5).collect::<Vec<_>>())
	                 .collect::<Vec<_>>();
	p(V2D(b));
	p(std::cmp::max(3, 8));
	p(std::cmp::max(32, 47));
	#[allow(non_snake_case)]
	let i = 1..=8;
	p(V2D(i.clone()
	       .map(|x| i.clone().map(|y| std::cmp::max(y, x)).collect_vec())
	       .collect_vec()));
	p(V2D(i.clone()
	       .map(|x| i.clone().map(|y| std::cmp::min(y, x)).collect_vec())
	       .collect_vec()));
	p(V2D(i.clone()
	       .map(|x| i.clone().map(|y:i32| x.pow(y as u32)).collect_vec())
	       .collect_vec()));
	println!("{:?}  {}", vec![2; 2].iter().fold(0, |x, y| x + y), 2 * 2);
	println!("{:?}  {}", vec![2; 3].iter().fold(0, |x, y| x + y), 2 * 3);
	println!("{:?}  {}", vec![2; 4].iter().fold(0, |x, y| x + y), 2 * 4);
	println!("{:?}  {}", vec![2; 5].iter().fold(0, |x, y| x + y), 2 * 5);
	println!("{:?}  {}", vec![2; 6].iter().fold(0, |x, y| x + y), 2 * 6);
	println!("{:?}  {}", vec![3; 8].iter().fold(0, |x, y| x + y), 3 * 8);

	println!(
	         "{:?}  {}",
	         vec![2; 2].iter().fold(1, |x, y| x * y),
	         2i32.pow(2)
	);
	println!(
	         "{:?}  {}",
	         vec![2; 3].iter().fold(1, |x, y| x * y),
	         2i32.pow(3)
	);
	println!(
	         "{:?}  {}",
	         vec![2; 4].iter().fold(1, |x, y| x * y),
	         2i32.pow(4)
	);
	println!(
	         "{:?}  {}",
	         vec![2; 5].iter().fold(1, |x, y| x * y),
	         2i32.pow(5)
	);
	println!(
	         "{:?}  {}",
	         vec![2; 6].iter().fold(1, |x, y| x * y),
	         2i32.pow(6)
	);
	println!(
	         "{:?}  {}",
	         vec![3; 8].iter().fold(1, |x, y| x * y),
	         3i32.pow(8)
	);
	p({ 1..=5 }.map(|x| x + 3).collect_vec());
	p({ 1..=5 }.map(|x| x + 4).collect_vec());
	p({ 4..=8 }.map(|x| x - 3).collect_vec());
	p({ 5..=9 }.map(|x| x - 4).collect_vec());
	p({ 1..=9 }.map(|x| x - 5).collect_vec());
	p({ 1..=9 }.map(|x| x + (-5)).collect_vec());
	p({ 0..=5 }.collect_vec());
	p({ 1..=6 }.map(|x| -1 + x).collect_vec());
	p({ 1..=8 }.map(|x| -8 + x).collect_vec());
	let s = { 1..9 }.map(|x| -5 + x);
	p(s.clone().map(|x| 1 + x).collect_vec());
	p(s.clone().map(|x| -2 + x).collect_vec());
	p(s.clone().zip(s.clone()).map(|(x, y)| x - y).collect_vec());
	p(s.clone().zip(s.clone()).map(|(x, y)| x + y).collect_vec());
	p(s.clone().map(|x| 2 * x).collect_vec());
	p(s.clone().zip(s.clone()).zip(s.clone()).map(|((x, y), z)| x + y + z).collect_vec());
	p(s.clone().map(|x| 3 * x).collect_vec());

	const I:RangeInclusive<i32> = 1..=9;
	p(I.collect_vec());
	let s = I.map(|x| I.map(|y| x - y).collect_vec()).collect_vec();
	p(V2D(s.clone()));
	p(V2D(s.clone()
	       .iter()
	       .map(|x| x.iter().rev().collect_vec())
	       .rev()
	       .collect_vec()));
	p(V2D(s.clone()
	       .iter()
	       .map(|x| x.iter().rev().collect_vec())
	       .collect_vec()));

	p(V2D(s.clone()
	       .iter()
	       .map(|x| x.iter().collect_vec())
	       .rev()
	       .collect_vec()));
	p(I.rev().collect_vec());
	let m = { 1..=6 }.map(|x| { 1..=6 }.map(|y| x - y).collect_vec()).collect_vec();
	p(V2D(m.clone()));
	p(m.clone()[2][3]);
	p(m.clone()[3][2]);
	p(m[2].clone());
	p(m.clone().iter().map(|x| x[2]).collect_vec());

	let p_ = vec![2, 3, 5, 7, 11];
	p(p_[3]);
	p(p_[1]);
	p(vec![2, 3, 5, 7, 11][1]);
	p(p_.iter()
	    .enumerate()
	    .filter(|x| match *x {
		    (i, _) if i == 0 || i == 2 || i == 4 => true,
	        _ => false,
	    })
	    .map(|(_, x)| *x)
	    .collect_vec());
	p(p_.iter()
	    .enumerate()
	    .filter(|x| match *x {
		    (i, _) if i == 0 || i == 2 || i == 4 => true,
	        _ => false,
	    })
	    .map(|(_, x)| *x)
	    .collect_vec());
	p(p_.iter()
	    .enumerate()
	    .filter(|x| match *x {
		    (_e @ 0..=3, _) => true,
	        _ => false,
	    })
	    .map(|(_, x)| *x)
	    .collect_vec());
	p(p_.iter()
	    .enumerate()
	    .filter(|x| match *x {
		    (_index @ 0..=4, _) => true,
	        _ => false,
	    })
	    .map(|(_, x)| *x)
	    .rev()
	    .collect_vec());
	p(V2D(m.clone()
	       .iter()
	       .enumerate()
	       .filter(|(x, _)| match *x {
		       0 | 1 => true,
	           _ => false,
	       })
	       .map(|(_, x)| {
		       x.iter()
		        .enumerate()
		        .filter(|(x, _)| match *x {
			        1 | 3 | 5 => true,
		            _ => false,
		        })
		        .map(|(_, x)| x)
		        .collect_vec()
	       })
	       .collect_vec()));
	p(V2D(m.clone()
	       .iter()
	       .enumerate()
	       .filter(|(x, _)| match *x {
		       0 | 2 => true,
	           _ => false,
	       })
	       .map(|(_, x)| x.clone())
	       .collect_vec()));
	p(V2D(m.clone()
	       .iter()
	       .map(|x| {
		       x.iter()
		        .enumerate()
		        .filter(|(x, _)| match *x {
			        1 | 3 | 5 => true,
		            _ => false,
		        })
		        .map(|(_, x)| *x)
		        .collect_vec()
	       })
	       .collect_vec()));

	let i = 1..=7;
	let a = i.clone().map(|x| i.clone().map(|y| x + y).collect_vec()).collect_vec();
	p(V2D(a));
	let j = { 1..=15 }.map(|x| x - 8);
	p(j.clone().collect_vec());
	p(V2D(j.clone()
	       .map(|x| j.clone().map(|y| x + y).collect_vec())
	       .collect_vec()));
	let m = i.clone().map(|x| i.clone().map(|y| x * y).collect_vec()).collect_vec();
	p(V2D(m.clone()));
	let n = j.clone().map(|x| j.clone().map(|y| x * y).collect_vec()).collect_vec();
	p(V2D(n.clone()));
	p(m.clone()
	   .iter()
	   .enumerate()
	   .filter(|(x, _)| match *x {
		   3 => true,
	       _ => false,
	   })
	   .map(|(_, x)| x)
	   .next()
	   .unwrap());
	p(n.clone().iter().nth(11).unwrap());
	let i = { 1..=13 }.map(|x| x - 7);
	p(i.clone().collect_vec());
	p(std::cmp::max(3, 5));
	p(std::cmp::max(3, -5));
	p(i.clone().map(|x| std::cmp::max(3, x)).collect_vec());
	p(i.clone().map(|x| std::cmp::max(-3, x)).collect_vec());
	p(V2D(i.clone()
	       .map(|x| i.clone().map(|y| std::cmp::max(x, y)).collect_vec())
	       .collect_vec()));
	p(V2D(i.clone()
	       .map(|x| i.clone().map(|y| std::cmp::min(x, y)).collect_vec())
	       .collect_vec()));
	p(match 3 == 8 {
		true => 1,
		_ => 0,
	});
	p(match 3 == 3 {
		true => 1,
		_ => 0,
	});
	p(match { -3 } == 3 {
		true => 1,
		_ => 0,
	});
	let i = 1..=5;
	p(i.clone().collect_vec());
	p(i.clone().rev().collect_vec());
	p(i.clone()
	   .zip(i.clone().rev())
	   .map(|(x, y)| match x == y {
		   true => 1u8,
	       _ => 0,
	   })
	   .collect_vec());

	let s = i.clone().map(|x| i.clone().map(|y| x - y).collect_vec()).collect_vec();
	p(V2D(s.clone()));
	p(V2D(s.clone()
	       .iter()
	       .rev()
	       .map(|x| x.iter().rev().collect_vec())
	       .collect_vec()));
	p(V2D(s.clone()
	       .iter()
	       .zip(s.clone().iter().rev().map(|x| x.iter().rev().collect_vec()))
	       .map(|(x, y)| {
		       x.clone()
		        .iter()
		        .zip(y.iter())
		        .map(|(x, y)| match *x == **y {
			        true => 1,
		            _ => 0,
		        })
		        .collect_vec()
	       })
	       .collect_vec()));
	p(V2D(s.clone()
	       .iter()
	       .zip(s.clone().iter().rev().map(|x| x.iter().rev().collect_vec()))
	       .map(|(x, y)| x.clone().iter().zip(y.iter()).map(|(x, y)| *x + **y).collect_vec())
	       .collect_vec()));
	p(V2D(s.clone()
	       .iter()
	       .zip(s.clone().iter().rev().map(|x| x.iter().rev().collect_vec()))
	       .map(|(x, y)| {
		       x.clone()
		        .iter()
		        .zip(y.iter())
		        .map(|(x, y)| match *x + **y == 0 {
			        true => 1,
		            _ => 0,
		        })
		        .collect_vec()
	       })
	       .collect_vec()));
	let m = i.clone().map(|x| i.clone().map(|y| x * y).collect_vec()).collect_vec();
	p(V2D(m.clone()));
	// p(m.clone());
	let trans_m = {
		let ravel =
			m.clone()
			 .into_iter()
			 .enumerate()
			 .map(|(iy, v)| {
				 v.into_iter().enumerate().map(|(ix, e)| /*flip*/(( ix,iy), e)).collect_vec()
			 })
			 .fold(Vec::with_capacity(m.len() * m[0].len()), |mut x, mut y| {
				 x.append(&mut y);
				 x
			 });
		let mut n_v2d = vec![Vec::new()];
		ravel.into_iter().for_each(|((y, x), e)| loop {
			                 match e {
				                 _ if n_v2d.len() <= y => n_v2d.push(Vec::new()),
			                     _ if n_v2d[y].len() <= x => n_v2d[y].push(0),
			                     _ => break n_v2d[y][x] = e,
			                 };
		                 });
		n_v2d
	};
	p(V2D(trans_m.clone()));
	p(V2D(m.clone()
	       .iter()
	       .zip(trans_m.clone().iter())
	       .map(|(x, y)| {
		       x.iter()
		        .zip(y.iter())
		        .map(|(a, b)| match a == b {
			        true => 1,
		            _ => 0,
		        })
		        .collect_vec()
	       })
	       .collect_vec()));
	p(V2D(m.clone()
	       .iter()
	       .zip(trans_m.clone().iter())
	       .map(|(x, y)| {
		       x.iter()
		        .zip(y.iter())
		        .map(|(a, b)| a - b)
		        .map(|c| match c == 0 {
			        true => 1,
		            _ => 0,
		        })
		        .collect_vec()
	       })
	       .collect_vec()));
	p(3 < 5);
	p(3 < 5);
	p(3 < 3);
	let n = { 1..=9 }.map(|x| x - 5);
	p(n.clone().collect_vec());
	p(n.clone().rev().collect_vec());
	p(n.clone()
	   .zip(n.clone().rev())
	   .map(|(x, y)| match x < y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	p(n.clone()
	   .rev()
	   .zip(n.clone())
	   .map(|(x, y)| match x < y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	p(n.clone()
	   .zip(n.clone().rev())
	   .map(|(x, y)| match x > y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	p(n.clone()
	   .rev()
	   .zip(n.clone())
	   .map(|(x, y)| match x > y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	let i = { 1..=7 }.map(|x| x - 4);
	p(i.clone().collect_vec());
	let r = i.clone().rev();
	p(r.clone().collect_vec());
	p(i.clone()
	   .zip(r.clone())
	   .map(|(x, y)| match x <= y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	p(i.clone()
	   .zip(r.clone())
	   .map(|(x, y)| match x < y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	p(i.clone()
	   .zip(r.clone())
	   .map(|(x, y)| match x == y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	p(i.clone()
	   .zip(r.clone())
	   .map(|(x, y)| match x >= y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	p(i.clone()
	   .zip(r.clone())
	   .map(|(x, y)| match x > y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	p(i.clone()
	   .zip(r.clone())
	   .map(|(x, y)| match x == y {
		   true => 1,
	       _ => 0,
	   })
	   .collect_vec());
	p(V2D({ 0..=1 }.map(|x| {
		               { 0..=1 }.map(|y| std::cmp::max(x, y)).collect_vec()
	               })
	               .collect_vec()));
	let x_ = 1..=5;
	let y_ = { 1..=5 }.rev();
	p(x_.clone()
	    .zip(y_.clone())
	    .map(|(x, y)| match x < y {
		    true => 1,
	        _ => 0,
	    })
	    .collect_vec());
	p(x_.clone()
	    .zip(y_.clone())
	    .map(|(x, y)| match x == y {
		    true => 1,
	        _ => 0,
	    })
	    .collect_vec());

	p((x_.clone()
	     .zip(y_.clone())
	     .map(|(x, y)| match x < y {
		     true => 1,
	         _ => 0,
	     })
	     .collect_vec()).iter()
	                .zip((x_.clone()
	                        .zip(y_.clone())
	                        .map(|(x, y)| match x == y {
		                        true => 1,
	                            _ => 0,
	                        })
	                        .collect_vec()).iter())
	                .map(|(x, y)| std::cmp::max(x, y))
	                .collect_vec());
p(x_.clone().zip(y_.clone()).map(|(x,y)|x<=y).collect_vec());
	p(V2D({ 0..=1 }.map(|x| {
		               { 0..=1 }.map(|y| std::cmp::min(x, y)).collect_vec()
	               })
	               .collect_vec()));
let w= vec![4,6,2,3,7];
p(w.clone().iter().map(|x|match 1<*x{true=>1,_=>0}).collect_vec());
}

fn p<T>(x:T)
	where T: std::fmt::Debug {
	println!("{:?}", x)
}
struct V2D<T>(Vec<Vec<T>>);
impl<'a, T> std::fmt::Debug for V2D<T>
	where Vec<T>: std::fmt::Debug,
	      T: std::fmt::Debug
{
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut string = String::new();
		string.push_str("\n");
		for i in 0usize..(self.0.len()) {
			string.push_str(&format!(
				"{}{:?}{}",
				match i {
					0 => "[",
					_ => " ",
				},
				self.0[i],
				match i == { self.0.len() - 1 } {
					true => "]\n",
					_ => "\n",
				}
			))
		}
		write!(f, "{}", string)
	}
}
