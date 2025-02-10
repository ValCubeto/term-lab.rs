use super::Size;
use std::io;
use std::mem::zeroed as empty_struct;
use std::os::windows::io::AsRawHandle;
use windows_sys::Win32::Foundation::{ HANDLE as Handle, INVALID_HANDLE_VALUE };
use windows_sys::Win32::System::Console::{
  GetConsoleMode as get_console_mode,
  CONSOLE_SCREEN_BUFFER_INFO as ConsoleScreenBufferInfo,
  GetConsoleScreenBufferInfo as get_console_screen_buffer_info,
};

pub fn size_of<H>(handle: &H) -> Option<Size>
where
  H: AsRawHandle
{
  let handle = handle.as_raw_handle() as Handle;
  if handle == INVALID_HANDLE_VALUE {
    return None;
  }
  if unsafe { get_console_mode(handle, &mut 0) } == 0 {
    return None;
  }
  // Create an empty struct to receive the info from the function
  let mut info: ConsoleScreenBufferInfo = unsafe { empty_struct() };
  if unsafe { get_console_screen_buffer_info(handle, &mut info) } == 0 {
    return None;
  }
  // Avoid negative values
  let rows = (info.srWindow.Bottom - info.srWindow.Top + 1).max(0) as u16;
  let cols = (info.srWindow.Right - info.srWindow.Left + 1).max(0) as u16;
  if rows > 0 && cols > 0 {
    Some(Size::new(rows, cols))
  } else {
    None
  }
}

pub fn stdin_size() -> Option<Size> {
  let handle = io::stdin().as_raw_handle() as Handle;
  if handle == INVALID_HANDLE_VALUE {
    return None;
  }
  if unsafe { get_console_mode(handle, &mut 0) } == 0 {
    return None;
  }
  size_of(&io::stdout())
}
