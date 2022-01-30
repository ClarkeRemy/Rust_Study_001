macro_rules! method {
	($object:ident . $method:ident | $( $val:tt )+) => {
		$($object.$method$val;)+
	};
}





pub fn 起動() {
	// HashMap

	use std::collections::HashMap as HM;

	let mut capitals = HM::new();

	const INPUT:[(&str, &str); 6] = [
	                                 ("Cook Islands", "Avarua"),
	                                 ("Fiji", "Suva"),
	                                 ("Kiribati", "South Tarawa"),
	                                 ("Niue", "Alofi"),
	                                 ("Tonga", "Nuku'alofa"),
	                                 ("Tuvala", "Funafuti"),
	];

	for i in &INPUT {
		capitals.insert(i.0, i.1);
	}

	println!("Capital of Tonga is: {}", capitals["Tonga"]);

	// BTree
	use std::collections::BTreeMap as BTM;

	let mut voc = BTM::new();

	const INVESTMENTS:[(u64, &str); 6] = [
	                                      (3_697_915, "Amsterdam"),
	                                      (1_300_405, "Middlburg"),
	                                      (540_000, "Enkhuizen"),
	                                      (469_400, "Delft"),
	                                      (266_868, "Hoorn"),
	                                      (173_000, "Rotterdam"),
	];

method!(
	voc.insert |
		(3_697_915, "Amsterdam") 
		(1_300_405, "Middlburg")
		(540_000, "Enkhuizen")
		(469_400, "Delft")
		(266_868, "Hoorn")
		(173_000, "Rotterdam")
);


let invest:BTM<u64,&str> = INVESTMENTS.iter().copied().collect();





// for i in &INVESTMENTS {
// voc.insert(i.0,i.1);
// }

for (guilders,kamer) in invest {
println!("chamber {} invested {}", kamer, guilders);
}

}
