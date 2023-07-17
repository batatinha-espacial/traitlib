pub use std::any::Any;
pub use std::borrow::{Borrow, BorrowMut, ToOwned};
pub use std::clone::Clone;
pub use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
pub use std::convert::{AsMut, AsRef, From, Into, TryFrom, TryInto};
pub use std::default::Default;
pub use std::error::Error;
pub use std::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex, Write as FmtWrite};
pub use std::future::{Future, IntoFuture};
pub use std::hash::{BuildHasher, Hash, Hasher};
pub use std::io::{BufRead, Read, Write, Seek};
pub use std::iter::{DoubleEndedIterator, ExactSizeIterator, Extend, FromIterator, FusedIterator, IntoIterator, Iterator, Product, Sum};
pub use std::marker::{Copy, Send, Sized, Sync, Unpin};
pub use std::net::ToSocketAddrs;
pub use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, DivAssign, Div, Drop, Fn, FnMut, FnOnce, IndexMut, Index, Mul, MulAssign, Neg, Not, RangeBounds, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
pub use std::panic::{RefUnwindSafe, UnwindSafe};
pub use std::slice::SliceIndex;
pub use std::str::FromStr;
pub use std::string::ToString;
pub use std::task::Wake;
use std::io::Result as IoResult;

pub trait ByteSerialize {
    fn byte_serialize(&self) -> Vec<u8>;
}
pub trait StrSerialize {
    fn str_serialize(&self) -> String;
}
pub trait JSONSerialize {
    fn json_serialize(&self) -> String;
}
pub trait XMLSerialize {
    fn xml_serialize(&self) -> String;
}
pub trait Sortable {
    fn sort_ascending(&mut self);
    fn sort_descending(&mut self);
}
pub trait Validate {
    fn is_valid(&self) -> bool;
}
pub trait Execute {
    fn exec(&mut self);
}
pub trait AsyncExecute {
    fn async_exec(&mut self) -> Box<dyn Future<Output = ()>>;
}
pub trait Encrypt {
    type Output;
    fn encrypt(&self, key: Vec<u8>) -> Self::Output;
    fn decrypt(encrypted: Self::Output, key: Vec<u8>) -> Self; 
}
pub trait Merge {
    fn merge(&self, other: &Self) -> Self;
}
pub trait Stream: Read + Write {
    fn copy_to<T: Stream>(&mut self, mut writer: T) -> IoResult<usize> {
        let mut total_copied = 0usize;
        let mut buf = [0u8; 8192];

        loop {
            let bytes = self.read(&mut buf)?;
            if bytes == 0 {
                break;
            }
            writer.write_all(&buf[..bytes])?;
            total_copied += bytes;
        }

        Ok(total_copied)
    }
    fn skip<const N: usize>(&mut self) -> IoResult<()> {
        let mut buf = [0u8; N];
        self.read(&mut buf)?;
        Ok(())
    }
    fn read_until(&mut self, delimiter: u8, buf: &mut Vec<u8>) -> IoResult<usize> {
        let mut total_read = 0usize;
        loop {
            let mut byte = [0u8; 1];
            self.read_exact(&mut byte)?;
            total_read += 1;
            if byte[0] == delimiter {
                break;
            }
            buf.push(byte[0]);
        }
        Ok(total_read)
    }
    fn read_line(&mut self, buf: &mut Vec<u8>) -> IoResult<usize> {
        Ok(self.read_until(b'\n', buf)?)
    }
    fn write_flush(&mut self, buf: &[u8]) -> IoResult<usize> {
        let wrote = self.write(buf)?;
        self.flush()?;
        Ok(wrote)
    }
    fn write_from_iterator<I: Iterator<Item = u8>>(&mut self, iterator: I) -> IoResult<()> {
        for byte in iterator {
            self.write_all(&[byte])?;
        }
        Ok(())
    }
    fn write_string(&mut self, string: String) -> IoResult<()>  {
        self.write_all(string.as_bytes())?;
        Ok(())
    }
}

impl<T: Read + Write> Stream for T {}

pub trait StreamSeek: Stream {
    fn seek(&mut self, pos: usize) -> IoResult<()>;
    fn seek_start(&mut self) -> IoResult<()>;
    fn seek_end(&mut self) -> IoResult<()>;
}

pub trait LineRead {
    fn read_line(&mut self) -> Vec<u8>;
}

pub trait ChunkedWrite {
    type Chunk;
    fn write_chunk(&mut self, chunk: Self::Chunk) -> IoResult<()>;
    fn write_all_chunks(&mut self, chunk: Vec<Self::Chunk>) -> IoResult<()>;
}

pub trait StreamTimeout: Stream {
    fn set_timeout(&mut self, milis: usize) -> IoResult<()>;
    fn clear_timeout(&mut self) -> IoResult<()>;
}

pub trait PacketStream {
    type Packet;
    fn send_packet(&mut self, packet: Self::Packet) -> IoResult<()>;
    fn receive_packet(&mut self) -> IoResult<Self::Packet>;
}

pub trait Encode {
    type Encoded;
    fn encode(&self) -> Self::Encoded;
    fn decode(encoded: Self::Encoded) -> Self;
}