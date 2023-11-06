<!-- cargo-rdme start -->

Convenience methods for encoding and decoding numbers in either big-endian
or little-endian.

Primitive integers implement [`ByteOrder`](https://docs.rs/bitendian/latest/bitendian/trait.ByteOrder.html).
```rust
use bitendian::ByteOrder;

let it: u16 = 256;
assert_eq!(ByteOrder::to_be_bytes(it), [1, 0]);
assert_eq!(ByteOrder::to_le_bytes(it), [0, 1]);
```

Extension methods provide convenient readers and writers.
```rust
use bitendian::{io::WriteExt as _, tokio::AsyncReadExt as _};

let mut buf = vec![];
buf.write_be(1u16)?;
let swapped = buf.as_slice().read_le().await?;
assert_eq!(256u16, swapped);
```

# Comparison with [`byteorder`].
- This crate leverages type inference to avoid [defining dozens of e.g write_uXX methods].
  ```rust
  use byteorder::{ReadBytesExt as _, BE, LE};
  use bitendian::io::ReadExt as _;
  use std::io;

  fn read_header(mut r: impl io::Read) -> io::Result<Header> {
      // before...
      Ok(Header {
          count: r.read_u16::<BE>()?,
                     // ^ this can be inferred
          offset: r.read_i32::<LE>()?
                            // ^ this could be a plain method
      })
      // after
      Ok(Header {
          count: r.read_be()?,
          offset: r.read_le()?,
      })
  }
  ```
- This crate supports run-time endianness.
- This crate supports [`futures::io`] and [`tokio::io`] via the `futures`
  and `tokio` features respectively.
- This crate only supports rust's built-in types, not, eg. [`u24`].
- Both crates support `#![no_std]` by disabling the default `std` feature.

[`byteorder`]: https://docs.rs/byteorder/1/byteorder/index.html
[defining dozens of e.g write_uXX methods]: https://docs.rs/byteorder/1/byteorder/trait.WriteBytesExt.html#method.write_u8
[`u24`]: https://docs.rs/byteorder/1/byteorder/trait.WriteBytesExt.html#method.write_u24
[`futures::io`]: https://docs.rs/futures/0.3/futures/io/
[`tokio::io`]: https://docs.rs/tokio/1/tokio/io/index.html

<!-- cargo-rdme end -->
