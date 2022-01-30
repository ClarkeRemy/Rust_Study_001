#[allow(dead_code)]
pub fn run() { render_mandelbrot(calculate_mandelbrot(1000, [-2.0, 1.0, -1.0, 1.0], [180, 50])) }

fn calculate_mandelbrot(max_iters:usize,
                        [x_min, x_max, y_min, y_max]:[f64; 4],
                        [width, height]:[usize; 2])
                        -> Vec<Vec<usize>> {
	fn mandelbrot_escape_val([cx, cy]:[f64; 2], max_iters:usize) -> usize {
		use num::complex::Complex as C;
		let mut z = C { re:0.0, im:0.0 };
		let c = C::new(cx, cy);

		for i in 0..=max_iters {
			match z.norm() {
				n if n > 2.0 => return i,
				_ => z = z * z + c,
			}
		}
		return max_iters;
	}

	let mut all_rows:Vec<Vec<usize>> = Vec::with_capacity(height);
	for img_y in 0..height {
		let mut row:Vec<usize> = Vec::with_capacity(width);
		for img_x in 0..width {
			const C_:fn([f64; 2], [usize; 2]) -> f64 =
				|[min, max], [img_dim, dim]| min + (max - min) * img_dim as f64 / dim as f64;
			let [cx, cy] = [
			                C_([x_min, x_max], [img_x, width]),
			                C_([y_min, y_max], [img_y, height]),
			];

			row.push(mandelbrot_escape_val([cx, cy], max_iters));
		}
		all_rows.push(row);
	}
	all_rows
}

fn render_mandelbrot(escape_vals:Vec<Vec<usize>>) {
	for row in escape_vals {
		let mut line = String::with_capacity(row.len());

		for column in row {
			line.push(match column {
				    0..=2 => ' ',
			        3..=5 => '.',
			        6..=10 => '•',
			        11..=30 => '*',
			        31..=100 => '+',
			        101..=200 => 'x',
			        201..=400 => '$',
			        401..=700 => '#',
			        _ => '%',
			    });
		}
		println!("{}", line);
	}
}
