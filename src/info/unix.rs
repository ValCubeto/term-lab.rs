use std::os::unix::io::AsFd;
use rustix::termios::{ isatty as is_a_tty, tcgetwinsize as get_win_size };
use super::Size;

pub fn size_of<Fd: AsFd>(stream: &Fd) -> Option<Size> {
  if !is_a_tty(stream) {
    return None;
  }
  let win_size = get_win_size(stream).ok()?;
  let (cols, rows) = (win_size.ws_col, win_size.ws_row);
  if cols > 0 && rows > 0 {
    Some(Size::new(rows, cols))
  } else {
    None
  }
}
