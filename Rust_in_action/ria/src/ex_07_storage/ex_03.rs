#![allow(dead_code,unused)]
macro_rules! print_all {
	($($text:tt)*  ) => {
		$(println!$text;)*
	};
}

pub fn 起動() {
	use {
		serde_derive::{Deserialize as 逆シリアル化, Serialize as シリアル化},
		String as 文字列,
	};

	#[derive(シリアル化, 逆シリアル化)]
	struct 街 {
		名前:文字列,
		人口:usize,
		緯度_経度:[f64; 2],
	}

	let calabar = 街 { 名前:文字列::from("Calabar"),
	                    人口:470_000,
	                    緯度_経度:[4.95, 8.33] };
	let (json, cbor, bincode):(文字列, Vec<u8>, Vec<u8>) =
		(serde_json::to_string(&calabar).unwrap(),
		 serde_cbor::to_vec(&calabar).unwrap(),
		 bincode::serialize(&calabar).unwrap());

print_all![
	("json: {}",&json)
	("cbor: {:?}",&cbor)
	("cbor (as UTF-8): {:?}",文字列::from_utf8_lossy(&cbor))
	("bincode: {:?}", &bincode)
	("bincode (as UTF-8): {:?}",文字列::from_utf8_lossy(&bincode))
];

}
