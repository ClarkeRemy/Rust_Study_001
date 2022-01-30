#![allow(dead_code)]
use {
	graphics::math::{add, mul_scalar, Vec2d},
	rand::prelude::*,
	std::{
		alloc::{GlobalAlloc, Layout, System},
		time::Instant,
	},
};

#[global_allocator]
static アロケーター:報告アロケーター = 報告アロケーター;

struct 報告アロケーター;

unsafe impl GlobalAlloc for 報告アロケーター {
	unsafe fn alloc(&self, layout:Layout) -> *mut u8 {
		let 開始 = Instant::now();
		let ptr = System.alloc(layout);
		let 終了 = Instant::now();
		let 時間 = 終了 - 開始;
		let 要求バイト = layout.size();

		eprintln!("{}\t{}", 要求バイト, 時間.as_nanos());
		ptr
	}
	unsafe fn dealloc(&self, ptr:*mut u8, layout:Layout) { System.dealloc(ptr, layout); }
}

struct 世界 {
	今の順番:u64,
	粒子等:Vec<Box<粒子>>,
	広さ_高さ:[f64; 2],
	rng:ThreadRng,
}
impl 世界 {
	fn new(広さ_高さ:[f64; 2]) -> 世界 {
		世界 { 今の順番:0,
		         粒子等:Vec::<Box<粒子>>::new(),
		         広さ_高さ,
		         rng:thread_rng() }
	}
	fn 形状を追加(&mut self, n:i32) {
		for _ in 0..n.abs() {
			self.粒子等.push(Box::new(粒子::new(&self)));
		}
	}
	fn 形状を削除(&mut self, n:i32) {
		for _ in 0..n.abs() {
			let mut 削除 = None;
			for (i, 一部粒子) in self.粒子等.iter().enumerate() {
				if 一部粒子.色[3] < 0.02 {
					削除 = Some(i);
				}
					break;
			}
			match 削除 {
				Some(i) => self.粒子等.remove(i),
				_ => self.粒子等.remove(0),
			};
		}
	}
	fn 更新(&mut self) {
		let n = self.rng.gen_range(-3..=3);
		match n {
			0 => self.形状を追加(n),
			_ => self.形状を削除(n),
		};

		self.粒子等.shrink_to_fit();
		for shape in &mut self.粒子等 {
			shape.更新();
		}
		self.今の順番 += 1;
	}
}

// #[derive(Clone)]
struct 粒子 {
	広さ_高さ:[f64; 2],
	位置:Vec2d<f64>,
	速度:Vec2d<f64>,
	加速度:Vec2d<f64>,
	色:[f32; 4],
}

impl 粒子 {
	fn new(引数世界:&世界) -> 粒子 {
		let mut rng = thread_rng();
		let [広さ, 高さ] = 引数世界.広さ_高さ;

		let [x, y] = [rng.gen_range(0.0..=広さ), 高さ];
		let [x_速度, y_速度] = [0.0, rng.gen_range(-2.0..0.0)];
		let [x_加速度, y_加速度] = [0.0, rng.gen_range(0.0..0.15)];

		粒子 { 広さ_高さ:[4.0; 2],
		         位置:[x, y].into(),
		         速度:[x_速度, y_速度].into(),
		         加速度:[x_加速度, y_加速度].into(),
		         色:[1.0, 1.0, 1.0, 0.99] }
	}
	fn 更新(&mut self) {
		self.速度 = add(self.速度, self.加速度);
		self.位置 = add(self.位置, self.速度);
		self.加速度 = mul_scalar(self.加速度, 0.7);
		self.色[3] *= 0.995;
	}
}

pub fn 起動() {
	use piston_window::*;
	let 広さ_高さ = [1280.0, 960.0];
	let mut window:PistonWindow =
		WindowSettings::new("particles", 広さ_高さ).exit_on_esc(true)
		                                           .build()
		                                           .expect("ウィンドウを作れませんでした。");

	let mut せかい = 世界::new(広さ_高さ);
	せかい.形状を追加(1000);

	while let Some(事象) = window.next() {
		せかい.更新();

		window.draw_2d(&事象, |ctx, renderer, _device| {
			      for s in &mut せかい.粒子等 {
				      // let 粒子 { 広さ_高さ: [広さ, 高さ],
				      //              位置: [x, y],
				      //              .. } = *s.clone();
				      let [[x, y], [広さ, 高さ]]:[[f64; 2]; 2] = [s.位置, s.広さ_高さ];
				      rectangle(s.色, [x, y, 広さ, 高さ], ctx.transform, renderer);
			      }
		      });
	}
}
