// cargo build ; qemu-system-x86_64 -drive format=raw,file=target\ria_os\debug\bootimage-ria_os.bin
#![feature(core_intrinsics)]
#![no_std]
#![no_main]
#![feature(lang_items)]

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
fn panic(_info:&core::panic::PanicInfo) -> ! {
	#[allow(unused_unsafe)]
	unsafe {
		core::intrinsics::abort();
	}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	// let text = b"Remy in the x86_64!";
	// let mut cursor = Cursor { position:0,
	//                           foreground:Color::BrightCyan,
	//                           background:Color::Black };
	// cursor.print(text);

	// let mut framebuffer = 0xb8000 as *mut u8;
	// unsafe {
	// 	framebuffer.offset(1).write_volatile(0x30);
	// }

	loop {
		x86_64::instructions::hlt();
	}
}

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
	Black = 0x0,
	Blue = 0x1,
	Green = 0x2,
	Cyan = 0x3,
	Red = 0x4,
	Mageta = 0x5,
	Brown = 0x6,
	Gray = 0x7,
	DarkGray = 0x8,
	BrightBlue = 0x9,
	BrightGreen = 0xA,
	BrightCyan = 0xB,
	BrightRed = 0xC,
	BrightMagenta = 0xD,
	Yellow = 0xE,
	White = 0xF,
}

struct Cursor {
	position:isize,
	foreground:Color,
	background:Color,
}

impl Cursor {
	fn color(&self) -> u8 {
		let fg = self.foreground as u8;
		let bg = (self.background as u8) << 4;
		fg | bg
	}

	fn print(&mut self, text:&[u8]) {
		let color = self.color();
		let framebuffer = 0xb8000 as *mut u8;
		for &character in text {
			unsafe {
				framebuffer.offset(self.position).write_volatile(character);
				framebuffer.offset(self.position + 1)
				           .write_volatile(color);
			}
			self.position += 2;
		}
	}
}
