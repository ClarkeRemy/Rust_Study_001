#![allow(unused, dead_code)]

#[derive(Debug)]
pub enum UpstreamError {
	IO(std::io::Error),
	Parsing(std::net::AddrParseError),
}
impl std::fmt::Display for UpstreamError {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}
impl std::error::Error for UpstreamError {}
impl From<std::io::Error> for UpstreamError {
	fn from(error:std::io::Error) -> Self { UpstreamError::IO(error) }
}
impl From<std::net::AddrParseError> for UpstreamError {
	fn from(error:std::net::AddrParseError) -> Self { UpstreamError::Parsing(error) }
}

pub fn run() -> Result<(), UpstreamError> {
	use UpstreamError as UpErr;
	let _f = std::fs::File::open("invisible.txt")?;
	let _localhost = "::1".parse::<std::net::Ipv6Addr>()?;

	Ok(())
}
