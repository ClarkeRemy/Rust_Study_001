use {
	ria_lib::{ActionKV, ByteStr, ByteString},
	std::collections::HashMap,
};

#[cfg(target_os = "windows")]
const USAGE:&str = "
Usage:
   akv_mem.exe FILE get KEY
   akv_mem.exe FILE delete KEY
   akv_mem.exe FILE insert KEY VALUE
   akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE:&str = "
Usage:
   akv_mem FILE get KEY
   akv_mem FILE delete KEY
   akv_mem FILE insert KEY VALUE
   akv_mem FILE update KEY VALUE
";

#[allow(unused)] // does this actually do anything?
fn store_index_on_disk(a:&mut ActionKV, index_key:&ByteStr) {
	a.index.remove(index_key);
	let index_as_bytes = bincode::serialize(&a.index).unwrap();
	a.index = std::collections::HashMap::new();
	a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
	const INDEX_KEY:&ByteStr = b"+index";

	let args:Vec<String> = std::env::args().collect();
	let (fname, action, key, maybe_value) = (args.get(1).expect(&USAGE),
	                                         args.get(2).expect(&USAGE).as_ref(),
	                                         args.get(3).expect(&USAGE).as_ref(),
	                                         args.get(4));
	let path = std::path::Path::new(&fname);

	let mut store = ActionKV::open(path).expect("unable to open file");
	store.load().expect("unable to load data");

	match action {
		"get" => {
			let index_as_bytes = store.find(&INDEX_KEY).unwrap().unwrap();
			let index:HashMap<ByteString, u64> = bincode::deserialize(&index_as_bytes.1).unwrap();
			match index.get(key) {
				// was store.get
				None => eprintln!("{:?} not found", key),
				Some(v) => println!("{:?}", v),
			}
		}

		"delete" => store.delete(key).unwrap(),
		"insert" => store.insert(key, maybe_value.expect(&USAGE).as_ref())
		                 .unwrap(),
		"update" => store.update(key, maybe_value.expect(&USAGE).as_ref())
		                 .unwrap(),
		_ => eprintln!("{}", &USAGE),
	}
}
