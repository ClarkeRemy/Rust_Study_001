use {
	byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt},
	crc::crc32,
	serde_derive::{Deserialize, Serialize},
	std::{
		collections::HashMap,
		fs::{File, OpenOptions},
		io::{self, prelude::*, BufReader, BufWriter, SeekFrom},
		path::Path,
	},
};

#[rustfmt::skip] macro_rules! types {
	  ($(   $alias_1st:ident $public_1st:vis  $base_type_1st:ty )? 
    $( | $alias:ident     $public:vis      $base_type:ty     )*)

=> {$( $public_1st     type $alias_1st      =  $base_type_1st  ;)?
    $( $public         type $alias          =  $base_type      ;)*}}

types!( ByteString pub Vec<u8> |  ByteStr pub [u8]);
// Normal rust syntax
// pub type ByteString = Vec<u8>;
// pub type ByteStr    = [u8]



#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
	pub key:ByteString,
	pub value:ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
	f:File,
	pub index:HashMap<ByteString, u64>,
}

impl ActionKV {
	pub fn open(path:&Path) -> io::Result<Self> {
		let f = OpenOptions::new().read(true)
		                          .write(true)
		                          .create(true)
		                          .append(true)
		                          .open(path)?;
		let index = HashMap::new();
		Ok(ActionKV { f, index })
	}

	pub fn load(&mut self) -> io::Result<()> {
		let mut f = BufReader::new(&mut self.f);

		loop {
			let current_position = f.seek(SeekFrom::Current(0))?;

			let maybe_kv = ActionKV::proccess_record(&mut f);
			let kv = match maybe_kv {
				Ok(kv) => kv,
				Err(err) =>
					break match err.kind() {
						io::ErrorKind::UnexpectedEof => Ok(()),
						_ => Err(err),
					},
			};
			self.index.insert(kv.key, current_position);
		}
	}

	fn proccess_record<R:Read>(f:&mut R) -> io::Result<KeyValuePair> {
		let saved_checksum = f.read_u32::<LittleEndian>()?;
		let key_len = f.read_u32::<LittleEndian>()?;
		let val_len = f.read_u32::<LittleEndian>()?;
		let data_len = key_len + val_len;

		let mut data = ByteString::with_capacity(data_len as usize);
		#[allow(unused_braces, unused_must_use)]
		{
			let _ = f.by_ref().take(data_len as u64).read_to_end(&mut data);
		};
		debug_assert_eq!(data.len(), data_len as usize);

		let checksum = crc32::checksum_ieee(&data);
		match checksum != saved_checksum {
			true => panic!(
			               "data corruption encountered ({:08x} != {:08x})",
			               checksum, saved_checksum
			),
			_ => {
				let value = data.split_off(key_len as usize);
				let key = data;

				Ok(KeyValuePair { key, value })
			}
		}
	}

	pub fn seek_to_end(&mut self, key:&ByteStr) -> io::Result<Option<ByteString>> {
		match self.index.get(key) {
			None => Ok(None),
			Some(position) => {
				let p = *position;
				Ok(Some(self.get_at(p)?.value))
			}
		}
	}

	pub fn get_at(&mut self, position:u64) -> io::Result<KeyValuePair> {
		let mut f = BufReader::new(&mut self.f);
		f.seek(SeekFrom::Start(position))?;
		Ok(ActionKV::proccess_record(&mut f)?)
	}

	pub fn find(&mut self, target:&ByteStr) -> io::Result<Option<(u64, ByteString)>> {
		let mut f = BufReader::new(&mut self.f);
		let mut found:Option<(u64, ByteString)> = None;

		loop {
			let position = f.seek(SeekFrom::Current(0))?;
			let maybe_kv = ActionKV::proccess_record(&mut f);
			let kv = match maybe_kv {
				Ok(kv) => kv,
				Err(err) =>
					break match err.kind() {
						io::ErrorKind::UnexpectedEof => Ok(found),
						_ => Err(err),
					},
			};

			match kv.key == target {
				true => found = Some((position, kv.value)),
				_ => continue,
			}
		}
		// break return
	}

	pub fn insert(&mut self, key:&ByteStr, value:&ByteStr) -> io::Result<()> {
		let position = self.insert_but_ignore_index(key, value)?;
		self.index.insert(key.to_vec(), position);
		Ok(())
	}

	pub fn insert_but_ignore_index(&mut self, key:&ByteStr, value:&ByteStr) -> io::Result<u64> {
		let mut f = BufWriter::new(&mut self.f);

		let [key_len, val_len]:[usize; 2] = [key.len(), value.len()];
		let mut tmp = ByteString::with_capacity(key_len + val_len);

		for byte in key {
			tmp.push(*byte);
		}
		for byte in value {
			tmp.push(*byte);
		}

		let checksum = crc32::checksum_ieee(&tmp);

		let (next_byte, current_position) = (SeekFrom::End(0), f.seek(SeekFrom::Current(0))?);

		f.seek(next_byte)?;
		f.write_u32::<LittleEndian>(checksum)?;
		f.write_u32::<LittleEndian>(key_len as u32)?;
		f.write_u32::<LittleEndian>(val_len as u32)?;
		f.write_all(&tmp)?;

		Ok(current_position)
	}

	#[inline]
	pub fn update(&mut self, key:&ByteStr, value:&ByteStr) -> io::Result<()> {
		self.insert(key, value)
	}

	pub fn delete(&mut self, key:&ByteStr) -> io::Result<()> { self.insert(key, b"") }
}
