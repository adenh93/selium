//! A collection of codecs to encode/decode messages into various formats.
//!
//! In `Selium`, messages are sent over the wire in a binary format, and thus, the server has no
//! indication of, or any desire to make sense of the data. This is perfectly suitable for the
//! server, but not very convenient for client users.
//!
//! To alleviate this issue, the `Selium` library makes use of codecs to encode produced messages
//! into bytes that the server can work with, and decode messages into a format that is useful to
//! consumers.
//!
//! Codecs are almost never used directly, but are provided as a configuration option while
//! constructing a [Subscriber](crate::Subscriber) or [Publisher](crate::Publisher) stream. The
//! [Subscriber](crate::Subscriber) or [Publisher](crate::Publisher) will then call the underlying
//! [encode](crate::traits::MessageEncoder::encode) or [decode](crate::traits::MessageDecoder::decode)
//! method in their respective [Sink](futures::Sink) or [Stream](futures::Stream) implementations when
//! producing or consuming messages.
//!
//! # Encoder
//!
//! An `Encoder` is used to encode an input into a sequence of bytes before being transmitted
//! over the wire.
//!
//! ### The MessageEncoder Trait
//!
//! The [MessageEncoder](crate::traits::MessageEncoder) trait is responsible for specifying the process of
//! receiving a generic input of type `Item`, and condensing it down into a [BytesMut](bytes::BytesMut)
//! value.
//!
//! [MessageEncoder](crate::traits::MessageEncoder) exposes a single method to implementors,
//! [encode](crate::traits::MessageEncoder::encode).

//!
//! # Decoder
//!
//! A `Decoder` is used to decode a sequence of bytes received over the wire into the target `Item`
//! type.
//!
//! ### The MessageDecoder Trait
//!
//! The [MessageDecoder](crate::traits::MessageDecoder) trait is responsible for specifying the
//! process of converting a [BytesMut](bytes::BytesMut) value into the target `Item` type.
//!
//! [MessageDecoder](crate::traits::MessageDecoder) exposes a single method to implementors,
//! [decode](crate::traits::MessageDecoder::decode).
//!
//! # Custom Codecs
//!
//! `Selium` aims to provide a suitable collection of codecs for various message payload formats,
//! including UTF-8 [String] encoding/decoding, various [serde] binary serialization formats, such
//! as [bincode], and many others.
//!
//! However, when the provided codecs are either not suitable for your needs, or lack support for
//! a specific serialization format, it is trivial to create a custom codec via the
//! [MessageEncoder](crate::traits::MessageEncoder) and
//! [MessageDecoder](crate::traits::MessageDecoder) traits.
//!
//! ## Example
//!
//! To give a contrived example, let's create a codec called `ColorCodec`, which will encode and
//! decode a [tuple] containing three [u8] values to describe a color.
//!
//! To begin, we'll create a struct called `ColorCodec`, and derive the [Default] and [Clone]
//! traits.
//!
//! ```
//! #[derive(Default, Clone)]
//! pub struct ColorCodec;
//! ```
//!
//! Next, we will implement the [MessageEncoder](crate::traits::MessageEncoder) and
//! [MessageDecoder](crate::traits::MessageDecoder) traits for our `ColorCodec` struct,
//! Let's leave them unimplemented for now.
//!
//! For the sake of convenience, a type alias `Color` has been defined to give meaning to the
//! unnamed [tuple].
//!
//! ```
//! # #[derive(Default, Clone)]
//! # pub struct ColorCodec;
//! use anyhow::Result;
//! use selium::traits::{MessageEncoder, MessageDecoder};
//! use bytes::{Bytes, BytesMut};
//!
//! type Color = (u8, u8, u8);
//!
//! impl MessageEncoder<Color> for ColorCodec {
//!     fn encode(&self, item: Color) -> Result<Bytes> {
//!         unimplemented!()
//!     }
//! }
//!
//! impl MessageDecoder<Color> for ColorCodec {
//!     fn decode(&self, buffer: &mut BytesMut) -> Result<Color> {
//!         unimplemented!()
//!     }
//! }
//! ```
//!
//! Now we can finish the implementations of the [encode](crate::traits::MessageEncoder::encode) and
//! [decode](crate::traits::MessageEncoder::encode) methods.
//!
//! Starting with the [encode](crate::traits::MessageEncoder::encode) method, we can push each element in
//! the tuple onto a [BytesMut](bytes::BytesMut) buffer. Be sure to reserve enough space prior to this operation.
//!
//! ```
//! # use anyhow::Result;
//! # use selium::traits::MessageEncoder;
//! # use bytes::{Bytes, BytesMut, BufMut};
//! # #[derive(Default, Clone)]
//! # pub struct ColorCodec;
//! # type Color = (u8, u8, u8);
//! impl MessageEncoder<Color> for ColorCodec {
//!     fn encode(&self, (r, g, b): Color) -> Result<Bytes> {
//!         let mut buffer = BytesMut::with_capacity(3);
//!
//!         buffer.put_u8(r);
//!         buffer.put_u8(g);
//!         buffer.put_u8(b);
//!
//!         Ok(buffer.into())
//!     }
//! }
//! ```
//!
//! Finally, we can complete the [decode](crate::traits::MessageDecoder::decode) method by popping three
//! [u8] values from the [BytesMut](bytes::BytesMut) buffer, and reconstructing the tuple.
//!
//! ```
//! # use anyhow::Result;
//! # use selium::traits::MessageDecoder;
//! # use bytes::{Buf, BytesMut};
//! # #[derive(Default, Clone)]
//! # pub struct ColorCodec;
//! # type Color = (u8, u8, u8);
//! impl MessageDecoder<Color> for ColorCodec {
//!     fn decode(&self, buffer: &mut BytesMut) -> Result<Color> {
//!         let r = buffer.get_u8();
//!         let g = buffer.get_u8();
//!         let b = buffer.get_u8();
//!
//!         Ok((r, g, b))
//!     }
//! }
//! ```
//!
//! Putting it all together, we have the following code, showing just how simple it is to create a
//! customized codec to suit your specific message format.
//!
//! ```
//! use anyhow::Result;
//! use selium::traits::{MessageEncoder, MessageDecoder};
//! use bytes::{Bytes, BytesMut, Buf, BufMut};
//!
//! #[derive(Default, Clone)]
//! pub struct ColorCodec;
//!
//! type Color = (u8, u8, u8);
//!
//! impl MessageEncoder<Color> for ColorCodec {
//!     fn encode(&self, (r, g, b): Color) -> Result<Bytes> {
//!         let mut buffer = BytesMut::with_capacity(3);
//!
//!         buffer.put_u8(r);
//!         buffer.put_u8(g);
//!         buffer.put_u8(b);
//!
//!         Ok(buffer.into())
//!     }
//! }
//!
//! impl MessageDecoder<Color> for ColorCodec {
//!     fn decode(&self, buffer: &mut BytesMut) -> Result<Color> {
//!         let r = buffer.get_u8();
//!         let g = buffer.get_u8();
//!         let b = buffer.get_u8();
//!
//!         Ok((r, g, b))
//!     }
//! }
//! ```

#[cfg(feature = "bincode")]
mod bincode_codec;
mod string_codec;

#[cfg(feature = "bincode")]
pub use bincode_codec::*;

pub use string_codec::*;
