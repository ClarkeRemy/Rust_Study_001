#[allow(dead_code)]
pub fn 起動() {
	let mut cpu = CPU { レジスター:[0; 16],
	                    格納の位置:0,
	                    格納:[0; 4096],
	                    スタック:[0; 16],
	                    スタックポインター:0 };

	cpu.レジスター[0] = 5;
	cpu.レジスター[1] = 10;

	(cpu.格納[0x000] = 0x21, cpu.格納[0x001] = 0x00);
	(cpu.格納[0x002] = 0x21, cpu.格納[0x003] = 0x00);

	(cpu.格納[0x100] = 0x80, cpu.格納[0x101] = 0x14);
	(cpu.格納[0x102] = 0x80, cpu.格納[0x103] = 0x14);
	(cpu.格納[0x104] = 0x00, cpu.格納[0x105] = 0xEE);

	cpu.実行();

	assert_eq!(cpu.レジスター[0], 45);

	println!("5 + (10 * 2) + (10 * 2) = {}", cpu.レジスター[0]);
}

struct CPU {
	レジスター:[u8; 16],
	格納の位置:usize,
	格納:[u8; 0x1000],
	スタック:[u16; 16],
	スタックポインター:usize,
}

impl CPU {
	fn 命令コードを読む(&self) -> u16 {
		let (位置, 格納) = (self.格納の位置, self.格納);
		let [命令バイト_1, 命令バイト_2] = [格納[位置] as u16, 格納[位置 + 1] as u16];

		命令バイト_1 << 8 | 命令バイト_2
	}

	fn 実行(&mut self) {
		loop {
			let 命令コード = self.命令コードを読む();
			self.格納の位置 += 2;

			let c = ((命令コード & 0xF000) >> 12) as u8;
			let x = ((命令コード & 0x0F00) >> 8) as u8;
			let y = ((命令コード & 0x00F0) >> 4) as u8;
			let d = ((命令コード & 0x000F) >> 0) as u8;

			let nnn = 命令コード & 0x0FFF; // addr
							   //let kk = (命令コード & 0x00FF) as u8;

			match (c, x, y, d) {
				(0, 0, 0, 0) => return (),
				(0, 0, 0xE, 0xE) => self.戻り(),
				(0x2, _, _, _) => self.呼び出し(nnn),
				(0x8, _, _, 0x4) => self.加算_xy(x, y),
				_ => todo!("命令コード {:04x}", 命令コード),
			}
		}
	}

	fn 呼び出し(&mut self, アドレス:u16) {
		let sp = self.スタックポインター;
		let スタック = &mut self.スタック;

		match sp > スタック.len() {
			true => panic!("スタック桁溢れ!"),
			_ => {
				スタック[sp] = self.格納の位置 as u16;
				self.スタックポインター += 1;
				self.格納の位置 = アドレス as usize;
			}
		}
	}

	fn 戻り(&mut self) {
		match self.スタックポインター {
			0 => panic!("スタック下位桁溢れ"),
			_ => {
				self.スタックポインター -= 1;
				self.格納の位置 = self.スタック[self.スタックポインター] as usize
			}
		}
	}

	fn 加算_xy(&mut self, x:u8, y:u8) {
		let [x, y] = [x as usize, y as usize];
		let [引数_1, 引数_2] = [self.レジスター[x], self.レジスター[y]];

		let (値, 桁溢れの発見) = 引数_1.overflowing_add(引数_2);
		self.レジスター[x] = 値;

		match 桁溢れの発見 {
			true => self.レジスター[0xF] = 1,
			_ => self.レジスター[0xF] = 0,
		}
	}
}
