/// Turns a rust string literal into a null-terminated `&[u8]`.
#[macro_export]
macro_rules! c_str {
	($text:expr) => {{
		concat!($text, '\0').as_bytes()
	}};
}