#![doc = include_str!("../README.md")]

pub mod styles;
pub mod info;

#[doc(hidden)]
#[macro_export]
macro_rules! def_strings {
  ($( $( #[$doc:meta] )? $name:ident = $value:expr; )*) => {
    $(
      $( #[$doc] )?
      pub const $name: &str = $value;
    )*
  };
}
