use cfg_if::cfg_if;
use std::fmt;
use std::io::{ Stdout, Stderr, Stdin };

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
}

macro_rules! impl_all {
  // Prevent bug when using self
  (
    $self:ident;
    $out_size:expr, $err_size:expr, $in_size:expr
  ) =>
  {
    impl TermInfo for Stdout {
      #[inline(always)]
      fn size(&mut $self) -> Option<Size> { $out_size }
    }
    impl TermInfo for Stderr {
      #[inline(always)]
      fn size(&mut $self) -> Option<Size> { $err_size }
    }
    impl TermInfo for Stdin {
      #[inline(always)]
      fn size(&mut $self) -> Option<Size> { $in_size }
    }
  };
}

cfg_if!(
  if #[cfg(windows)] {
    mod windows;
    use windows::{ size_of, stdin_size };
    impl_all!(self; size_of(self), size_of(self), stdin_size());
  } else if #[cfg(unix)] {
    mod unix;
    use unix::size_of;
    impl_all!(self; size_of(self), size_of(self), size_of(self));
  } else {
    impl_all!(self; None, None, None);
  }
);
