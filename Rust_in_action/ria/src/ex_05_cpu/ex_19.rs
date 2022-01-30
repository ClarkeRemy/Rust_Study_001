#[allow(dead_code)]
pub fn 起動() {
	let mut cpu = CPU { 現在の運転:0,
	                    レジスター:[0; 2] };
	cpu.現在の運転 = 0x8014;
	cpu.レジスター[0] = 5;
	cpu.レジスター[1] = 10;

	cpu.実行();
	assert_eq!(cpu.レジスター[0], 15);
	println!("5 + 10 = {}", cpu.レジスター[0])
}

struct CPU {
	現在の運転:u16,
	レジスター:[u8; 2],
}

impl CPU {
	fn 命令コードを読む(&self) -> u16 { self.現在の運転 }

	fn 実行(&mut self) {
		// {loop
		let 命令コード = self.命令コードを読む();

		let c = ((命令コード & 0xF000) >> 12) as u8;
		let x = ((命令コード & 0x0F00) >> 8) as u8;
		let y = ((命令コード & 0x00F0) >> 4) as u8;
		let d = ((命令コード & 0x000F) >> 0) as u8;

		match (c, x, y, d) {
			(0x8, _, _, 0x4) => self.加算_xy(x, y),
			_ => todo!("命令コード　{:04x}", 命令コード),
		}
		//}
	}
	fn 加算_xy(&mut self, x:u8, y:u8) {
		self.レジスター[x as usize] += self.レジスター[y as usize]
	}
}
