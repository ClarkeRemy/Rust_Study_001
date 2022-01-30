#![allow(dead_code)]
#![allow(unused)]

	macro_rules! print_all {
 ($($text:tt)*) => {
  $(println!$text;)*
 };
}

	use std::{convert::TryInto, mem::ManuallyDrop};

	static B:[u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
	static C:[u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

	pub fn 起動() {
		#[allow(unused)]
		use std::{
			borrow::Cow,
			ffi::CStr,
			mem::{size_of, size_of_val},
			os::raw::c_char,
		};

		let a:usize = 42;
		let b = &B;
		let c:Box<[u8]> = Box::new(C);
		print_all!(
		["a (an unsigned integer):"                       ]
		["  location:  {:p}"      , &a                    ]
		["  size:      {:?} bytes", size_of::<usize>()    ]
		["  value:     {:?}"      , a                     ]
		[]
		["b (reference to B):"                            ]
		["  location:  {:p}"      , &b                    ]
		["  size:      {:?} bytes", size_of::<&[u8; 10]>()]
		["  points to: {:p}"      , b                     ]
		[]
		["c (a \"box\" for C):"                           ]
		["  location:  {:p}"      , &c                    ]
		["  size:      {:?} bytes", size_of::<Box<[u8]>>()]
		["  points to: {:p}"      , c                     ]
		[]
		["B (an array of 10 bytes):"                      ]
		["  location:  {:p}"      , &B                    ]
		["  size:      {:?} bytes", size_of::<[u8;10]>()  ]
		["  value:     {:?}"      , B                     ]
		[]
		["C (an array of 11 bytes):"                      ]
		["  location:  {:p}"      , &C                    ]
		["  size:      {:?} bytes", size_of::<[u8;11]>()  ]
		["  value:     {:?}"      , C                     ]
		[]
		);

		let a_ = 42;
		let (b_ptr, c_ptr) = (&B as *const u8 as *mut u8, &C as *const u8 as *const c_char);
		let (b_, c_) = unsafe {
			(String::from_raw_parts(b_ptr, B.len(), B.len()),
			 CStr::from_ptr(c_ptr).to_string_lossy())
		};
println!("a: {}, b: {}, c: {}", a_,b_,c_);
let mdc = ManuallyDrop::new((b_,/*c_*/)); // Cow didn't take ownership
		// let b_s = &B[0..10];
		// println! {"size_of\nref arr:{:?},\nslice  :{:?},",size_of::<&[u8;10]>(),size_of::<&[u8]>()};
		// println! {"size_of_val\nref arr:{:?},\nslice  :{:?},",size_of_val(b),size_of_val(b_s)};

		// let a = [0u16; 10];
		// let a_ref:&[u16; 10] = &a;
		// let a_slice:&[u16] = &a[1..3];
		// let a_ref_repr:[usize; 1] = unsafe { core::mem::transmute(a_ref) };
		// let a_slice_repr:[usize; 2] = unsafe { core::mem::transmute(a_slice) };
		// dbg!(a_ref_repr, a_slice_repr);

		// let x:*const [u16] = std::ptr::slice_from_raw_parts(30 as *const u16, 30);
		// let y:Box<[i32]> = Box::new([3; 3]);

		// let z:Box<[i32; 3]> = y.try_into().unwrap();
	}

