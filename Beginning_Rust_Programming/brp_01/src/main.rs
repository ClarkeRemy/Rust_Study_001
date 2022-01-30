extern crate crossterm;
extern crate rand;
use crossterm::{
	style::{Colorize, ContentStyle, StyledContent},
	terminal::{Clear, ClearType::All},
	ExecutableCommand,
};
use rand::random;
use std::{
	fs::File,
	io::{stdout, BufRead, BufReader, Write},
	{env, thread, time::Duration},
};

const DIMENSION: usize = 50;
const CYCLES: usize = 20;
const MILLIS: u64 = 100;

struct World([[u8; DIMENSION]; DIMENSION]);
impl World {
	fn rand(&mut self) {
		for row in 0..DIMENSION {
			for col in 0..DIMENSION {
				if random() {
					self.0[row][col] = 1;
				}
			}
		}
	}
	fn census(&self) -> u16 {
		let mut count = 0_u16;
		for row in 0..DIMENSION {
			for col in 0..DIMENSION {
				if self.0[row][col] == 1 {
					count = count + 1_u16;
				}
			}
		}
		count
	}
	fn new() -> World {
		World([[0u8; DIMENSION]; DIMENSION])
	}
	fn poplate_from_file(&mut self, filename: String) -> () {
		let file = File::open(filename).expect("could not open file, filename not found");
		let reader = BufReader::new(file);

		let mut pairs: Vec<(usize, usize)> = Vec::new();
		for (_, line) in reader.lines().enumerate() {
			let l = line.unwrap();

			let nums: Result<Vec<usize>, _> = l.split_whitespace().map(|w| w.parse()).collect();
			match nums {
				Ok(v) if ((v.len() == 2) && (v[0] < DIMENSION) && (v[1] < DIMENSION)) => {
					pairs.push((v[0], v[1]))
				}
				_ => (),
			}
		}
		let mut new_world = World::new();
		for row in 0..DIMENSION {
			for col in 0..DIMENSION {
				new_world.0[row][col] = 0;
			}
		}
		for (x, y) in pairs {
			new_world.0[x][y] = 1;
		}
		self.0 = new_world.0;
	}
	fn record_living(&self) -> String {
		let mut record: String = String::new();
		for row in 0..DIMENSION {
			for col in 0..DIMENSION {
				if self.0[row][col] == 1 {
					record.push_str(&row.to_string());
					record.push(' ');
					record.push_str(&col.to_string());
					record.push('\n');
				}
			}
		}
		record
	}
}
impl Iterator for World {
	type Item = World;

	fn next(&mut self) -> Option<World> {
		let mut new_world = World::new();
		for row in 1..(DIMENSION - 1) {
			for col in 1..(DIMENSION - 1) {
				let mut count = 0;
				for k in 0..3 {
					for l in 0..3 {
						if !(k == 1 && l == 1) {
							count = count + self.0[row + k - 1][col + l - 1];
						}
					}
				}
				match (count, self.0[row][col]) {
					(2, 1) | (3, 1) | (3, 0) => new_world.0[row][col] = 1,
					_ => (),
				}
			}
		}
		Some(new_world)
	}
}
impl std::fmt::Display for World {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		for row in 0..DIMENSION {
			for col in 0..DIMENSION {
				match self.0[row][col] {
					1 => write!(formatter, "{}", &" ".on_red())?,
					_ => write!(
						formatter,
						"{}",
						&StyledContent::new(ContentStyle::new(), " ")
					)?,
				}
			}
			writeln!(formatter)?;
		}
		Ok(())
	}
}

fn main() {
	let mut world = World::new();
	let args: Vec<String> = env::args().collect();
	let mut cycles = CYCLES;
	if args.len() < 2 {
		world.rand();
	} else {
		world.poplate_from_file(env::args().nth(1).unwrap());
		match env::args().nth(2) {
			Some(e) => match e.parse::<usize>() {
				Ok(n) => cycles = n + 1,
				_ => (),
			},
			_ => (),
		}
	}
	for generation in 0..cycles {
		cycle(generation, &mut world)
	}
	match File::create("output.txt") {
		Ok(mut out) => match out.write_all(world.record_living().as_bytes()) {
			Ok(_)=>println!("wrote output.txt"),
			_=>println!("failed to write output")
		},
		_ => println!("failed to write output"),
	};
}

fn cycle(generation: usize, world: &mut World) {
	let mut stdout = stdout();
	stdout.execute(Clear(All)).unwrap();
	println!(
		"{}{} Population at generation {} is {}",
		format!("{}", &world),
		" ".on_blue(),
		generation,
		world.census()
	);
	stdout.flush().unwrap();
	*world = world.next().unwrap();
	thread::sleep(Duration::from_millis(MILLIS));
}
