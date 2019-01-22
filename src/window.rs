#![windows_subsystem = "windows"]
extern crate winapi;
use std::io::Error as Window_Error;

// include!("logger.rs");
use logger;

fn win32_string( value : &str ) -> Vec<u16> {
	use std::os::windows::prelude::*;
	std::ffi::OsStr::new( value ).encode_wide().chain( std::iter::once( 0 ) ).collect()
}

pub struct Window 
{
	handle : winapi::shared::windef::HWND,
}

	// type HINSTANCE = winapi::shared::minwindef::HINSTANCE; //this seems less than ideal, too many namespaces imo
#[cfg(windows)]
pub fn create_window(title : &str, name : &str) -> Result<Window, Window_Error>
{
	logger::write("create_window");
	let name = win32_string(name);
	let title = win32_string(title);
	unsafe {
		let hinstance = winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null_mut());
		use window::winapi::um::winuser::*;
		use std::ptr::*;
		let wnd_class = WNDCLASSW {
			style : CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
			lpfnWndProc : Some( DefWindowProcW ),
			hInstance : hinstance,
			lpszClassName : name.as_ptr(),
			cbClsExtra : 0,
			cbWndExtra : 0,
			hIcon: null_mut(),
			hCursor: null_mut(),
			hbrBackground: null_mut(),
			lpszMenuName: null_mut(),
		};
	
		RegisterClassW(&wnd_class);

		let handle = CreateWindowExW(
			0,
			name.as_ptr(),
			title.as_ptr(),
	 		WS_OVERLAPPEDWINDOW | WS_VISIBLE,
			CW_USEDEFAULT, //start x
			CW_USEDEFAULT, //start y
			CW_USEDEFAULT, //width
			CW_USEDEFAULT, //height
			null_mut(),
			null_mut(),
			hinstance,
			null_mut() 
		);

		if handle.is_null() {
			Err( Window_Error::last_os_error() )
		} else {
			Ok( Window{ handle } )
		}
	}
}

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Window_Error> {
	use std::ffi::OsStr;
	use std::iter::once;
	use std::os::windows::ffi::OsStrExt;
	use std::ptr::null_mut;
	use window::winapi::um::winuser::{MB_OK, MessageBoxW};
	let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
	let ret = unsafe {
		MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
	};
	if ret == 0 { Err(Window_Error::last_os_error()) }
	else { Ok(ret) }
}

#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Window_Error> {
	println!("{}", msg);
	Ok(())
}

pub fn handle_message( window : &mut Window ) -> bool {
	unsafe {
		use window::winapi::um::winuser::*;
		let mut message : MSG = std::mem::uninitialized();
		if GetMessageW( &mut message as *mut MSG, window.handle, 0, 0 ) > 0 {
			TranslateMessage( &message as *const MSG );
			DispatchMessageW( &message as *const MSG );
			true
		} else {
			false
		}
	}
}