#![allow(dead_code, unused)]
macro_rules! println_all {
	($($text:tt)*) => {
		$(println!$text;)*
	};
}

pub fn 起動() {
	// 依存性
	use {
		kernel32::{GetCurrentProcess, GetCurrentProcessId, GetSystemInfo, VirtualQueryEx},
		std::mem::zeroed,
		winapi::{
			DWORD, HANDLE, LPSYSTEM_INFO, LPVOID, MEMORY_BASIC_INFORMATION, PVOID, SIZE_T,
			SYSTEM_INFO,
		},
	};
	const 記憶情報大きさ:usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

	// 開始
	let mut プロセス情報準備:SYSTEM_INFO = unsafe { zeroed() };
	unsafe { GetSystemInfo(&mut プロセス情報準備 as LPSYSTEM_INFO) };
	let [最低, 最高]:[LPVOID; 2] = [
	                                プロセス情報準備.lpMinimumApplicationAddress,
	                                プロセス情報準備.lpMaximumApplicationAddress,
	];

	let (
	     プロセス識別,
	     プロセス,
	     [最低_app_アドレス, 最高_app_アドレス],
	     mut 基本アドレス,
	     mut プロセス情報,
	     mut 記憶情報,
	):(DWORD, HANDLE, [LPVOID; 2], PVOID, SYSTEM_INFO, MEMORY_BASIC_INFORMATION) = unsafe {
		(GetCurrentProcessId(), GetCurrentProcess(), [最低, 最高], zeroed(), zeroed(), zeroed())
	};

	println_all! {
		("\nプロセス情報")
		("{:?} @ {:p}", プロセス識別,プロセス)
		("{:#?}", プロセス情報)
		("最低: {: >16p}\n最高: {: >16p}",最低_app_アドレス,最高_app_アドレス)
		("\nloop 開始\n")
	}

	loop {
		let rc:SIZE_T = unsafe {
			VirtualQueryEx(
			               プロセス,
			               基本アドレス,
			               &mut 記憶情報,
			               記憶情報大きさ as SIZE_T,
			)
		};
		match rc {
			0 => break,
			_ => {
				println!("{:#?}", 記憶情報);
				基本アドレス = ((基本アドレス as u64) + 記憶情報.RegionSize) as PVOID;
			}
		}
//end Loop
	}
}
