If you're looking for a library to encode and decode [LEB128] in Rust,
you're probably looking for:

  - https://crates.io/crates/leb128
  - https://crates.io/crates/scroll
  - https://crates.io/crates/protobuf

Not this library.

Note that we punt on encoding signed numbers, since there are several
approaches used in the wild.  Ideally this library would provide all
such approaches.

[LEB128]: https://en.wikipedia.org/wiki/LEB128
