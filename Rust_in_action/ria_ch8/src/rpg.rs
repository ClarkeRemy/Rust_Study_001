#![allow(unused,dead_code)]
use rand::{self, seq::SliceRandom, Rng};

macro_rules! derive_def_struct {
	    (#:$drv:tt  $( | $p:vis    $make_type:ident

$(   {  $($f1:ident : $typ1:ty,)*  }   )?
$( $((  $($tup1:ty,            )*  ))?;)?  )*)

=> {$(#[derive$drv]   $p struct $make_type

$(   {  $($f1       : $typ1   ,)*  }   )?
$( $((  $($tup1,               )*  ))?;)?  )*};
}

derive_def_struct!( #:(Debug) | Dwarf {} | Elf {} | Human {});

#[derive(Debug)]
enum Thing {
	Sword,
	Trinket,
}

trait Enchanter: std::fmt::Debug {
	fn competency(&self) -> f64;
fn enchant(&self, thing:&mut Thing) {
		let probability_of_success = self.competency();
		let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);

		print!("{:?} mutters incoherently. ", self);
		match spell_is_successful {
			true => println!("The {:?} glows brightly.", thing),
			_ => println!("The {:?} fizzles, then into a worthless triket.", thing),
		}
	}
}

macro_rules! impl_Enchanter {
	($(| $typ:ty : $comp:literal )*) => {
$(impl Enchanter for $typ {fn competency(&self) -> f64 {$comp}})*
};}

impl_Enchanter!(| Dwarf : 0.5 | Elf : 0.95 | Human : 0.8);

pub fn run() {
let mut it = Thing::Sword;

let (d,e,h)=(Dwarf {},Elf {},Human{});
let party: Vec<&dyn Enchanter> = vec![&d,&h,&e];
let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

spellcaster.enchant(&mut it);
}