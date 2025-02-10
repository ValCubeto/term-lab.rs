use super::Size;
use std::os::windows::io::{ AsHandle, AsRawHandle, BorrowedHandle, RawHandle };
use windows_sys::Win32::Foundation::{ HANDLE as Handle, INVALID_HANDLE_VALUE };
use windows_sys::Win32::System::Console::{
  GetStdHandle as get_std_handle,
  STD_HANDLE as StdHandle,
  STD_OUTPUT_HANDLE,
  STD_ERROR_HANDLE,
  STD_INPUT_HANDLE,
  COORD as Coords,
  SMALL_RECT as SmallRect,
  CONSOLE_SCREEN_BUFFER_INFO as ConsoleScreenBufferInfo,
  GetConsoleScreenBufferInfo as get_console_screen_buffer_info,
};

fn size_of(handle: StdHandle) -> Option<Size> {
  let handle = get_std_handle(STD_OUTPUT_HANDLE);
  let handle = unsafe { BorrowedHandle::borrow_raw(handle) };
  let handle = handle.as_raw_handle() as Handle;
  if handle == INVALID_HANDLE_VALUE {
    return None;
  }
  // Create an empty struct to receive the info from the function
  let coords = Coords { X: 0, Y: 0 };
  let mut info = ConsoleScreenBufferInfo {
    dwSize: coords,
    dwCursorPosition: coords,
    dwMaximumWindowSize: coords,
    wAttributes: 0,
    srWindow: SmallRect {
      Left: 0,
      Top: 0,
      Right: 0,
      Bottom: 0
    }
  };
  if unsafe { get_console_screen_buffer_info(handle, &mut info) } == 0 {
    return None;
  }
  // Avoid negative values
  let rows = (info.srWindow.Bottom - info.srWindow.Top).max(0);
  let cols = (info.srWindow.Right - info.srWindow.Left).max(0);
  Some(Size::new(rows as u16, cols as u16))
}

pub fn stdout_size() -> Option<Size> {
  size_of(STD_OUTPUT_HANDLE)
}

pub fn stderr_size() -> Option<Size> {
  size_of(STD_ERROR_HANDLE)
}

pub fn stdin_size() -> Option<Size> {
  size_of(STD_INPUT_HANDLE)
}
