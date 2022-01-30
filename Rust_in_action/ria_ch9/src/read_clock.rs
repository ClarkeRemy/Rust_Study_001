#![allow(dead_code, unused)]
use std::io::Read;

#[cfg(not(windows))] use libc;
use {
	byteorder::{BigEndian, ReadBytesExt},
	chrono::{DateTime, Duration as ChronoDuration, Local, TimeZone, Timelike, Utc},
	clap::{App, Arg},
	std::{mem::zeroed, net::UdpSocket, time::Duration},
};

const NTP_MESSAGE_LENGTH:usize = 48;
const NTP_TO_UNIX_SECONDS:i64 = 2_208_988_800;
const LOCAL_ADDR:&'static str = "0.0.0.0:12300";

#[derive(Default, Debug, Copy, Clone)]
struct NTPTimestamp {
	seconds:u32,
	fraction:u32,
}
struct NTPMessage {
	data:[u8; NTP_MESSAGE_LENGTH],
}

#[derive(Debug)]
struct NTPResult {
	t1:DateTime<Utc>,
	t2:DateTime<Utc>,
	t3:DateTime<Utc>,
	t4:DateTime<Utc>,
}

impl NTPResult {
	fn offset(&self) -> i64 { ((self.t2 - self.t1) + (self.t4 - self.t3)).num_milliseconds() / 2 }
	fn delay(&self) -> i64 { ((self.t4 - self.t1) - (self.t3 - self.t2)).num_milliseconds() }
}
impl From<NTPTimestamp> for DateTime<Utc> {
	fn from(ntp:NTPTimestamp) -> Self {
		let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
		let nsecs = (ntp.fraction as f64 * 1e9 / 2_f64.powi(32)) as u32;
		let all = (secs, nsecs);
		// Utc.timestamp(secs, nsecs)
		Utc.timestamp(secs, nsecs)
	}
}
impl From<DateTime<Utc>> for NTPTimestamp {
	fn from(utc:DateTime<Utc>) -> Self {
		let seconds = (utc.timestamp() + NTP_TO_UNIX_SECONDS) as u32;
		let fraction = (utc.nanosecond() as f64 * 2_f64.powi(32) / 1e9) as u32;
		NTPTimestamp { seconds, fraction }
	}
}
impl NTPMessage {
	fn new() -> Self { NTPMessage { data:[0; NTP_MESSAGE_LENGTH] } }
	fn client() -> Self {
		const VERSION:u8 = 3; // 0000_0011
		const MODE:u8 = 3;
		let mut msg = NTPMessage::new();
		msg.data[0] | VERSION << 3 | MODE;
		msg
	}
	fn parse_timestamp(&self, i:usize) -> Result<NTPTimestamp, std::io::Error> {
		let mut reader = &self.data[i..i + 8];
		let seconds = reader.read_u32::<BigEndian>()?;
		let fraction = reader.read_u32::<BigEndian>()?;
		Ok(NTPTimestamp { seconds, fraction })
	}
	fn rx_time(&self) -> Result<NTPTimestamp, std::io::Error> { self.parse_timestamp(32) }
	fn tx_time(&self) -> Result<NTPTimestamp, std::io::Error> { self.parse_timestamp(40) }
}

fn weighted_mean(values:&[f64], weights:&[f64]) -> f64 {
	let [mut sum, mut sum_of_weights]:[f64; 2] = [0.0, 0.0];
	for (v, w) in values.iter().zip(weights) {
		sum += v * w;
		sum_of_weights += w;
	}
	sum / sum_of_weights
}

fn ntp_roundtrip(host:&str, port:u16) -> Result<NTPResult, std::io::Error> {
	let (destination, timeout) = (format!("{}:{}", host, port), Duration::from_secs(1));
	let [request, mut response] = [NTPMessage::client(), NTPMessage::new()];

	let message = request.data;

	let udp = UdpSocket::bind(LOCAL_ADDR)?;
	udp.connect(&destination).expect("unable to connect");

	let t1 = Utc::now();
	udp.send(&message)?;
	udp.set_read_timeout(Some(timeout))?;
	udp.recv_from(&mut response.data)?;
	let t4 = Utc::now();
	let t2:DateTime<Utc> = response.rx_time().unwrap().into();
	let t3:DateTime<Utc> = response.tx_time().unwrap().into();

	Ok(NTPResult { t1, t2, t3, t4 })
}

fn check_time() -> Result<f64, std::io::Error> {
	const NTP_PORT:u16 = 123;

	let servers = [
	               "time.nist.gov",
	               "time.apple.com",
	               "time.euro.com",
	               "time.google.com",
	               "time2.google.com",
	               "time.windows.com",
	];
	let mut times = Vec::with_capacity(servers.len());

	for &server in servers.iter() {
		print!("{} =>", server);

		let calc = ntp_roundtrip(&server, NTP_PORT);

		match calc {
			Ok(time) => {
				println!(" {}ms away from local system time", time.offset());
				times.push(time);
			}
			Err(_) => println!(" ? [response took too long]"),
		}
	}

	let mut offsets = Vec::with_capacity(servers.len());
	let mut offset_weights = Vec::with_capacity(servers.len());

	for time in &times {
		let offset = time.offset() as f64;
		let delay = time.delay() as f64;

		let weight = 1_000_000.0 / delay.powi(2);
		if weight.is_finite() {
			offsets.push(offset);
			offset_weights.push(weight);
		}
	}
	let avg_offset = weighted_mean(&offsets, &offset_weights);

	Ok(avg_offset)
}

struct Clock;
impl Clock {
	fn get() -> DateTime<Local> { Local::now() }
	#[allow(dead_code)]
	#[cfg(windows)]
	fn set<Tz:TimeZone>(t:DateTime<Tz>) {
		use chrono::{Datelike, Timelike};

		use {
			kernel32::SetSystemTime,
			winapi::{SYSTEMTIME, WORD},
		};

		let t = t.with_timezone(&Local);

		let dow = {
			use chrono::Weekday::*;
			match t.weekday() {
				Mon => 1,
				Tue => 2,
				Wed => 3,
				Thu => 4,
				Fri => 5,
				Sat => 6,
				Sun => 0,
			}
		};
		const B:u32 = 1_000_000_000;
		let ns = match t.nanosecond() {
			leap if leap > B => leap - B,
			no_leap => no_leap,
		};
		let systime = SYSTEMTIME { wYear:t.year() as WORD,
		                           wMonth:t.month() as WORD,
		                           wDayOfWeek:dow as WORD,
		                           wDay:t.day() as WORD,
		                           wHour:t.hour() as WORD,
		                           wMinute:t.minute() as WORD,
		                           wSecond:t.second() as WORD,
		                           wMilliseconds:(ns / B) as WORD };
		let systime_ptr = &systime as *const SYSTEMTIME;
		unsafe {
			SetSystemTime(systime_ptr);
		}
	}
	#[cfg(not(windows))]
	fn set<Tz:TimeZone>(t:DateTime<Tz>) {
		use libc::{settimeofday, suseconds_t, time_t, timeval, timezone};

		let t = t.with_timezone(&Local);
		let mut u:timeval = unsafe { zeroed() };

		u.tv_sec = t.timestamp() as time_t;
		u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

		unsafe {
			let mock_tz:*const timezone = std::ptr::null();
			settimeofday(&u as *const timeval, mock_tz);
		}
	}
}

pub fn run() {
	let app = App::new("clock")
		.version("0.1")
		.about("Get an (aspirationally) sets the time.")
		.after_help("Note: Unix timestamps are parsed as whole seconds since 
1st January 1970 0:00:00 UTC. for more accuracy, use another format.")
		.arg(Arg::with_name("action").takes_value(true)
			                            .possible_values(&["get","set","check-ntp"])
			                            .default_value("get"))
		.arg(Arg::with_name("std").short("s").long("use-standard")
			                         .takes_value(true).possible_values(&["rfc2822","rfc3339","timestamp"])
			                         .default_value("rfc3339"))
		.arg(Arg::with_name("datetime").help("When <action> is set, apply <datetime>. 
Otherwise, ignore."));

	let args = app.get_matches();

	let action = args.value_of("action").unwrap();
	let std = args.value_of("std").unwrap();

	match action {
		"set" => {
			let parser = match std {
				"rfc2822" => DateTime::parse_from_rfc2822,
				"rfc3339" => DateTime::parse_from_rfc3339,
				_ => unimplemented!(),
			};

			let t_ = args.value_of("datetime").unwrap();
			let err_msg = format!("Unable to parse {} according to {}", t_, std);
			let t = parser(t_).expect(&err_msg);

			Clock::set(t);
		}
		"check-ntp" => {
			let offset = check_time().unwrap() as isize;
			let adjust_ms = offset.signum() * offset.abs().min(200) / 5;
			let adjust_ms_ = ChronoDuration::milliseconds(adjust_ms as i64);
			let now:DateTime<Utc> = Utc::now() + adjust_ms_;
			Clock::set(now)
		}
		_ => (),
	};

	let maybe_error = std::io::Error::last_os_error();
	let os_error_code = &maybe_error.raw_os_error();
	match os_error_code {
		Some(0) => (),
		None => (),
		_ => eprintln!("Unable to set the time: {:?}", maybe_error),
	}

	let now = Clock::get();
	match std {
		"timestamp" => println!("{}", now.timestamp()),
		"rfc2822" => println!("{}", now.to_rfc2822()),
		"rfc3339" => println!("{}", now.to_rfc3339()),
		_ => unreachable!(),
	}
}
