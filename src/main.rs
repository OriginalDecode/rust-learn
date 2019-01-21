// #![cfg(all(windows, target_arch = "x86_64"))]
#[macro_use]
extern crate lazy_static;

mod vector;
mod window;
mod logger;

#[cfg(windows)]
fn main() 
{
	let mut window = window::create_window("Rustc Window", "rustcwindow").unwrap();
	loop 
	{
		if !window::handle_message( &mut window ) 
		{
			break;
		}
		logger::write_log("Hello there!");
	}
}