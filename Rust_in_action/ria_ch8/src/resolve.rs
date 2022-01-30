#![allow(unused, dead_code)]
pub fn run() {
	use {
		clap::{App, Arg},
		rand::random,
		std::{
			net::{SocketAddr, UdpSocket},
			time::Duration,
		},
		trust_dns::{
			op::{Message, MessageType, OpCode, Query},
			rr::{domain::Name, record_type::RecordType},
			serialize::binary::*,
		},
	};

	let app = App::new("resolve").about("A simple to use DNS resolver")
	                             .arg(Arg::with_name("dns-server").short("s")
	                                                              .default_value("1.1.1.1"))
	                             .arg(Arg::with_name("domain-name").required(true))
	                             .get_matches();

	let [domain_name_raw, dns_server_raw]:[&str; 2] = [
	                                                   app.value_of("domain-name").unwrap(),
	                                                   app.value_of("dns-server").unwrap(),
	];

	let (mut request_as_bytes, mut response_as_bytes):(Vec<u8>, [u8; 512]) =
		(Vec::with_capacity(512), [0; 512]);

	let domain_name = Name::from_ascii(&domain_name_raw).unwrap();
	let msg = Message::new().set_id(random::<u16>())
	                        .set_message_type(MessageType::Query)
	                        .add_query(Query::query(domain_name, RecordType::A))
	                        .set_op_code(OpCode::Query)
	                        .set_recursion_desired(true)
	                        .to_owned();
	let mut encoder = BinEncoder::new(&mut request_as_bytes);
	msg.emit(&mut encoder).unwrap();

	let localhost = UdpSocket::bind("0.0.0.0:0").expect("cannot bind to local socket");
	let timeout = Duration::from_secs(3);

	localhost.set_read_timeout(Some(timeout)).unwrap();
	localhost.set_nonblocking(false).unwrap();

	let dns_server:SocketAddr = format!("{}:53", dns_server_raw).parse()
	                                                            .expect("invalid address");
	let _amt = localhost.send_to(&request_as_bytes, dns_server)
	                    .expect("timeout reached");

	let (_amt, _remote) = localhost.recv_from(&mut response_as_bytes)
	                               .expect("timeout reached");

	let dns_message = Message::from_vec(&response_as_bytes).expect("unable to parse response");

	for answer in dns_message.answers() {
		match answer {
			a if a.record_type() == RecordType::A => {
				let ip = answer.rdata()
				               .to_ip_addr()
				               .expect("invalid IP address received");
				println!("{}", ip.to_string());
			}
			_ => (),
		}
	}
}
