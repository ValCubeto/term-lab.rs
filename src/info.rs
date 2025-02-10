use cfg_if::cfg_if;
use std::fmt;
use std::io::{ Stdout, Stderr, Stdin, stdout, stdin, Write, Read };

#[derive(Debug, Clone, Copy)]
pub struct Size {
  pub width: u16,
  pub height: u16,
}

impl fmt::Display for Size {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}x{}", self.width, self.height)
  }
}

impl Size {
  #[inline(always)]
  pub fn new(width: u16, height: u16) -> Self {
    Self { width, height }
  }
}

/// Trait to get the size of the terminal
pub trait TermInfo {
  /// Returns the size of this stream in the terminal
  fn size(&mut self) -> Option<Size>;
  /// Returns whether the terminal supports ANSI escape codes
  fn is_ansi_enabled(&mut self) -> bool;
}

macro_rules! impl_all {
  // Prevent bug when using self
  (
    $self:ident;
    $out_size:expr, $err_size:expr, $in_size:expr;
    $out_ansi:expr, $err_ansi:expr
  ) =>
  {
    impl TermInfo for Stdout {
      #[inline(always)]
      fn size(&mut $self) -> Option<Size> { $out_size }
      #[inline(always)]
      fn is_ansi_enabled(&mut $self) -> bool { $out_ansi }
    }
    impl TermInfo for Stderr {
      #[inline(always)]
      fn size(&mut $self) -> Option<Size> { $err_size }
      #[inline(always)]
      fn is_ansi_enabled(&mut $self) -> bool { $err_ansi }
    }
    impl TermInfo for Stdin {
      #[inline(always)]
      fn size(&mut $self) -> Option<Size> { $in_size }
      #[inline(always)]
      /// Always false
      fn is_ansi_enabled(&mut $self) -> bool { false }
    }
  };
}

cfg_if! {
  if #[cfg(windows)] {
    mod windows;
    use windows::{ stdout_size, stderr_size, stdin_size };
    size_impl!(self;
      stdout_size(), stderr_size(), stdin_size();
      stdout_ansi(), stderr_ansi()
    );
  } else if #[cfg(unix)] {
    mod unix;
    use unix::{ size_of, ansi_enabled_in };
    impl_all!(self;
      size_of(self), size_of(self), size_of(self);
      ansi_enabled_in(self).is_ok(), ansi_enabled_in(self).is_ok()
    );
  } else {
    impl_all!(self;
      None, None, None,
      false, false, false
    );
  }
}
