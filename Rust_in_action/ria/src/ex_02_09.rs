#[allow(dead_code)]
pub fn run() {
	let context_lines = 2;
	let needle = "oo";
	// let search_term = "picture";
	let haystack = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

	let mut tags:Vec<usize> = Vec::new();
	let mut ctx:Vec<Vec<(usize, String)>> = Vec::new();

	for (i, line) in haystack.lines().enumerate() {
		match line.contains(needle) {
			true => {
				tags.push(i);
				ctx.push(Vec::with_capacity(2 * context_lines + 1))
			}
			_ => (),
		}
	}
	if tags.is_empty() {
		return;
	}
	for (i, line) in haystack.lines().enumerate() {
		for (j, tag) in tags.iter().enumerate() {
			if (i >= tag.saturating_sub(context_lines)) && (i <= { tag + context_lines }) {
				ctx[j].push((i, String::from(line)))
			}
		}
	}
	for local_ctx in ctx.iter() {
		for &(i, ref line) in local_ctx.iter() {
			println!("{}: {}", { i + 1 }, line);
		}
	}
}
