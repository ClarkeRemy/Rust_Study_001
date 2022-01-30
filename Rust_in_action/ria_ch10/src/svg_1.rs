#![allow(dead_code,unused)]
use {
	rayon::prelude::*,
	svg::{
		node::element::{
			path::{Command, Data, Position},
			Path, Rectangle,
		},
		Document,
	},
};

// Listing 10.18
pub fn run() {
	let args = std::env::args().collect::<Vec<String>>();
	let input = args.get(1).unwrap();
	let default_filename = format!("{}.svg", input);
	let save_to = args.get(2).unwrap_or(&default_filename);

	let operations = parse(input);
	let path_data = convert(&operations);
	let document = generate_svg(path_data);
	svg::save(save_to, &document).unwrap();
}

macro_rules! CONST_LIST {
	($($typ:ty [ $($name:ident = $val:expr ,)*];)*) => {
		$($(const $name : $typ = $val ;)*)*
	};
}

CONST_LIST! {
	isize [
		WIDTH = 400,
		HEIGHT = WIDTH,
		HOME_X = HEIGHT/2,
		HOME_Y = WIDTH/2,];
	usize [
		STROKE_WIDTH = 5,];
}

#[derive(Debug, Clone, Copy)]
enum Operation {
	Foward(isize),
	TurnLeft,
	TurnRight,
	Home,
	Noop(u8),
}
#[derive(Debug, Clone, Copy)]
enum Orientation {
	North,
	East,
	West,
	South,
}
#[derive(Debug)]
struct Artist {
	x:isize,
	y:isize,
	heading:Orientation,
}

fn parse(input:&str) -> Vec<Operation> {
	input.as_bytes().par_iter()
	     .map(|byte| {
		     use Operation::*;
		     match byte {
			     b'0' => Home,
		         b'1'..=b'9' => {
			         let distance = (byte - 0x30) as isize;
			         Foward(distance * (HEIGHT / 10))
		         }
		         b'a' | b'b' | b'c' => TurnLeft,
		         b'd' | b'e' | b'f' => TurnRight,
		         _ => Noop(*byte),
		     }
	     })
	     .collect()
}
fn convert(operations:&Vec<Operation>) -> Vec<Command> {
	let mut turtle = Artist::new();
	let mut path_data = Vec::<Command>::with_capacity(1 + operations.len());
	path_data.push(Command::Move(Position::Absolute, (HOME_X, HOME_Y).into()));

	for op in operations {
		use Operation::*;
		match *op {
			Foward(distance) => turtle.forward(distance),
			TurnLeft => turtle.turn_left(),
			TurnRight => turtle.turn_right(),
			Home => turtle.home(),
			Noop(byte) => eprintln!("warning: illegal byte encountered: {:?}", byte),
		};
		path_data.push(Command::Line(Position::Absolute, (turtle.x, turtle.y).into()));
		turtle.wrap();
	}
	path_data
}
impl Artist {
	fn new() -> Artist {
		Artist { heading:Orientation::North,
		         x:HOME_X,
		         y:HOME_Y }
	}
	fn home(&mut self) {
		self.x = HOME_X;
		self.y = HOME_Y;
	}
	fn forward(&mut self, distance:isize) {
		use Orientation::*;
		match self.heading {
			North => self.y += distance,
			South => self.y -= distance,
			West => self.x += distance,
			East => self.x -= distance,
		}
	}
	fn turn_right(&mut self) {
		use Orientation::*;
		self.heading = match self.heading {
			North => East,
			South => West,
			West => North,
			East => South,
		}
	}
	fn turn_left(&mut self) {
		use Orientation::*;
		self.heading = match self.heading {
			North => West,
			South => East,
			West => South,
			East => North,
		}
	}
	fn wrap(&mut self) {
		use Orientation::*;
		match self.x {
			x if x < 0 => {
				self.x = HOME_X;
				self.heading = West;
			}
			x if x > WIDTH => {
				self.x = HOME_X;
				self.heading = East;
			}
			_ => (),
		};
		match self.y {
			y if y < 0 => {
				self.y = HOME_Y;
				self.heading = North;
			}
			y if y > WIDTH => {
				self.y = HOME_Y;
				self.heading = South;
			}
			_ => (),
		}
	}
}
fn generate_svg(path_data:Vec<Command>) -> Document {
	let background = Rectangle::new().set("x", 0)
	                                 .set("y", 0)
	                                 .set("width", WIDTH)
	                                 .set("height", HEIGHT)
	                                 .set("fill", "#ffffff");

	let border = background.clone()
	                       .set("fill-opacity", "0.0")
	                       .set("stroke-width", 3 * STROKE_WIDTH);

	let sketch = Path::new().set("fill", "none")
	                        .set("stroke", "#2f2f2f")
	                        .set("stroke-width", STROKE_WIDTH)
	                        .set("stroke-opacity", "0.9")
	                        .set("d", Data::from(path_data));

	let document = Document::new().set("viewBox", (0, 0, HEIGHT, WIDTH))
	                              .set("height", HEIGHT)
	                              .set("width", WIDTH)
	                              .set("style", "style=\"outline: 5px solid #800000;\"")
	                              .add(background)
	                              .add(sketch)
	                              .add(border);

	document
}
