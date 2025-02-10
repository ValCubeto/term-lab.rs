use std::io::{ self, Write, Read };
use std::os::unix::io::{ AsFd, AsRawFd };
use std::sync::mpsc;
use std::time::{ Duration, Instant, SystemTime };
use std::thread::sleep;
use rustix::termios::{ isatty as is_a_tty, tcgetwinsize as get_win_size };
use nix::fcntl::{ fcntl, FcntlArg, OFlag };
use termion::cursor::DetectCursorPos;
use termion::AsyncReader;
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

fn stdin_nonblocking() -> std::io::Result<i32> {
    let fd = io::stdin().as_raw_fd();
    // Save original mode
    let original_flags = fcntl(fd, FcntlArg::F_GETFL)?;
    fcntl(fd, FcntlArg::F_SETFL(OFlag::from_bits_truncate(original_flags) | OFlag::O_NONBLOCK))?;
    Ok(original_flags)
}

fn stdin_mode(original_flags: i32) -> std::io::Result<()> {
  let fd = io::stdin().as_raw_fd();
  fcntl(fd, FcntlArg::F_SETFL(OFlag::from_bits_truncate(original_flags)))?;
  Ok(())
}

fn async_stdin_until(delimiter: u8) -> mpsc::Receiver<io::Result<u8>> {
  let (send, recv) = mpsc::channel();
  std::thread::spawn(move || for b in io::stdin().bytes() {
    match b {
      Ok(byte) if byte == delimiter || send.send(Ok(byte)).is_err() => return,
      Ok(..) => {}
      Err(..) => return,
    }
  });
  recv
}

fn read_cursor_pos() -> io::Result<u8> {
  let delimiter = b'R';
  let mut stdin = async_stdin_until(delimiter);

  let mut buf: [u8; 1] = [0];
  let mut read_chars = Vec::new();

  let timeout = Duration::from_millis(100);
  let now = SystemTime::now();

  // Either consume all data up to R or wait for a timeout.
  while now.elapsed().unwrap() < timeout {
    let n = match stdin.try_recv() {
      Ok(Ok(b)) => Ok(b),
      Ok(Err(e)) => Err(e),
      Err(..) => Err()
    }?;
    if n == delimiter {
      break;
    }
    if stdin.read(&mut buf)? > 0 {
      read_chars.push(buf[0]);
    }
  }

  if read_chars.is_empty() {
    return Err(Error::new(
      ErrorKind::Other,
      "Cursor position detection timed out.",
    ));
  }
}

pub fn ansi_enabled_in<Fd>(stream: &mut Fd) -> io::Result<()>
where
  Fd: AsFd + Write
{
  let other_err = Err(io::Error::new(io::ErrorKind::Other, ""));
  if !is_a_tty(&stream) {
    return other_err;
  }
  // Save cursor position
  let save_cursor = "\x1b[s";
  // Ask the terminal for cursor position
  let get_size = "\x1b[6n";
  let _ = stream.write(save_cursor.as_bytes())?;
  let _ = stream.write(get_size.as_bytes())?;
  // Print all
  stream.flush()?;

  read_cursor_pos();

  // Restore cursor position
  let restore_pos = "\x1b[u";
  let _ = stream.write(restore_pos.as_bytes())?;
  
  // Clear the line as nothing happened
  let printed = save_cursor.len() + get_size.len() + response.len() + restore_pos.len();
  // let _ = stream.write(format!("\r{}\r", " ".repeat(printed)).as_bytes())?;

  // Already flushed successfully
  stream.flush()?;

  dbg!(&response);
  // Check if response contains expected cursor position format
  if response.bytes().next() == Some(0x1b) {
    Ok(())
  } else {
    other_err
  }
}
