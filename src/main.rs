// #![cfg(all(windows, target_arch = "x86_64"))]


mod vector;
mod window;
mod logger;



#[cfg(windows)]
fn main() 
{
	let mut log : logger::Logger = logger::new_log();

	let mut window = window::create_window("Rustc Window", "rustcwindow").unwrap();
	loop 
	{
		if !window::handle_message( &mut window ) 
		{
			break;
		}
		log.write_log("Hello there!");
	}

	
}