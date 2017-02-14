#![cfg_attr(nightly, feature(foo))]

///This is cursor over text. It is fast, but has no access to previous lexemes.
pub mod stream_lexer;

pub mod line;
pub use line::Line;

pub mod fragment;
pub use fragment::Fragment;

pub mod error;
pub use error::Error;
