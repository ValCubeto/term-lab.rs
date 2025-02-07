#![doc = include_str!("../README.md")]

pub mod styles;

#[doc(hidden)]
#[macro_export]
macro_rules! def_strings {
  ($( $name:ident = $value:expr; )*) => {
    $( pub const $name: &str = $value; )*
  };
}
