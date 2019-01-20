// #![cfg(all(windows, target_arch = "x86_64"))]


mod vector;
mod window;

use std::io::Error;

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
	}
}