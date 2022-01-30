mod http_get;
mod resolve;
mod rpg;
mod use_reqwest;
mod will_fail;
mod mac_gen;

fn main()
//
// -> Result<(), will_fail::UpstreamError>
{
	// use_reqwest::run();
	// rpg::run();
	// http_get::run();
	// resolve::run();
	// return will_fail::run(); // change return of main to
	mac_gen::run();
}
