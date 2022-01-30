#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(warnings)]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use blog_os::{memory, println};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
 use blog_os::{
  allocator,
  memory::{self, BootInfoFrameAllocator},
 };
 use x86_64::{
  structures::paging::{Page, Translate},
  VirtAddr,
 };

 println!("Hello World{}", "!");
 println!("{}", TANUKI);
 println!("Tanuki says hello");

 blog_os::init();

 let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
 let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
 let mut mapper = unsafe { memory::init(phys_mem_offset) };

 // let page = Page::containing_address(VirtAddr::new(
 //  //
 //  0,
 // ));
 // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

 // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
 // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };


 allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

 println!("here");

 let heap_value = Box::new(41);
 println!("heap_value at {:p}", heap_value);

 let mut vec = Vec::new();
 for i in 0..500 {
  vec.push(i);
 }
 println!("vec at {:p}", vec.as_slice());

 let reference_counted = Rc::new(vec![1, 2, 3]);
 let cloned_reference = reference_counted.clone();
 println!(
  "current reference count is {}",
  Rc::strong_count(&cloned_reference)
 );
 core::mem::drop(reference_counted);
 println!(
  "reference count is {} now",
  Rc::strong_count(&cloned_reference)
 );

 #[cfg(test)]
 test_main();

 println!("It did not crash!");

 blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
 println!("{}", info);
 blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
 blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
 assert_eq!(1, 1);
}

const TANUKI: &str = "
............###........###..........
..........###,##.......##.##........
..........##,,,##......##,,##.......
..........##,,##,,####,##,##........
###........####...###....###........
####......###/.  .#/##.  .###.......
/////....###//..../  / ...####/.....
./////....#######/..../#######......
..####.....##################.......
";
