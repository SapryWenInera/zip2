#![allow(missing_docs)]

use std::io;
use std::io::{Read, Write};

#[cfg(feature = "tokio")]
use std::future::Future;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Provides high level API for reading from a stream.
pub mod stream {
    pub use crate::read::stream::*;
}
/// Types for creating ZIP archives.
pub mod write {
    use crate::write::{FileOptionExtension, FileOptions};
    /// Unstable methods for [`FileOptions`].
    pub trait FileOptionsExt {
        /// Write the file with the given password using the deprecated ZipCrypto algorithm.
        ///
        /// This is not recommended for new archives, as ZipCrypto is not secure.
        fn with_deprecated_encryption<S>(self, password: S) -> Self
        where
            S: AsRef<[u8]>;
    }
    impl<'k, T: FileOptionExtension> FileOptionsExt for FileOptions<'k, T> {
        fn with_deprecated_encryption<S>(self, password: S) -> Self
        where
            S: AsRef<[u8]>,
        {
            self.with_deprecated_encryption(password.as_ref())
        }
    }
}

/// Helper methods for writing unsigned integers in little-endian form.
pub trait LittleEndianWriteExt: Write {
    fn write_u16_le(&mut self, input: u16) -> io::Result<()> {
        self.write_all(&input.to_le_bytes())
    }

    fn write_u32_le(&mut self, input: u32) -> io::Result<()> {
        self.write_all(&input.to_le_bytes())
    }

    fn write_u64_le(&mut self, input: u64) -> io::Result<()> {
        self.write_all(&input.to_le_bytes())
    }

    fn write_u128_le(&mut self, input: u128) -> io::Result<()> {
        self.write_all(&input.to_le_bytes())
    }
}

impl<W: Write> LittleEndianWriteExt for W {}

/// Async version of LittleEndianWriteExt
#[cfg(feature = "tokio")]
pub trait AsyncLittleEndianWriteExt: AsyncWrite + Unpin {
    fn write_u16_le(&mut self, input: u16) -> impl Future<Output = io::Result<()>> + Send
    where
        Self: Send,
    {
        async move { self.write_all(&input.to_le_bytes()).await }
    }

    fn write_u32_le(&mut self, input: u32) -> impl Future<Output = io::Result<()>> + Send
    where
        Self: Send,
    {
        async move { self.write_all(&input.to_le_bytes()).await }
    }

    fn write_u64_le(&mut self, input: u64) -> impl Future<Output = io::Result<()>> + Send
    where
        Self: Send,
    {
        async move { self.write_all(&input.to_le_bytes()).await }
    }

    fn write_u128_le(&mut self, input: u128) -> impl Future<Output = io::Result<()>> + Send
    where
        Self: Send,
    {
        async move { self.write_all(&input.to_le_bytes()).await }
    }
}

#[cfg(feature = "tokio")]
impl<W: AsyncWrite + Unpin> AsyncLittleEndianWriteExt for W {}

/// Helper methods for reading unsigned integers in little-endian form.
pub trait LittleEndianReadExt: Read {
    fn read_u16_le(&mut self) -> io::Result<u16> {
        let mut out = [0u8; 2];
        self.read_exact(&mut out)?;
        Ok(u16::from_le_bytes(out))
    }

    fn read_u32_le(&mut self) -> io::Result<u32> {
        let mut out = [0u8; 4];
        self.read_exact(&mut out)?;
        Ok(u32::from_le_bytes(out))
    }

    fn read_u64_le(&mut self) -> io::Result<u64> {
        let mut out = [0u8; 8];
        self.read_exact(&mut out)?;
        Ok(u64::from_le_bytes(out))
    }
}

impl<R: Read> LittleEndianReadExt for R {}

/// Async version of LIttleEndianReadExt
#[cfg(feature = "tokio")]
pub trait AsyncLittleEndianReadExt: AsyncRead + Unpin {
    fn read_u16_le(&mut self) -> impl Future<Output = io::Result<u16>> + Send
    where
        Self: Send,
    {
        async {
            let mut out = [0u8; 2];
            self.read_exact(&mut out).await?;
            Ok(u16::from_le_bytes(out))
        }
    }

    fn read_u32_le(&mut self) -> impl Future<Output = io::Result<u32>> + Send
    where
        Self: Send,
    {
        async {
            let mut out = [0u8; 4];
            self.read_exact(&mut out).await?;
            Ok(u32::from_le_bytes(out))
        }
    }

    fn read_u64_le(&mut self) -> impl Future<Output = io::Result<u64>> + Send
    where
        Self: Send,
    {
        async {
            let mut out = [0u8; 8];
            self.read_exact(&mut out).await?;
            Ok(u64::from_le_bytes(out))
        }
    }
}

#[cfg(feature = "tokio")]
impl<R: AsyncRead + Unpin> AsyncLittleEndianReadExt for R {}
