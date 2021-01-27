/*!
```rudra-poc
[target]
crate = "libp2p-deflate"
version = "0.27.0"

[[target.peer]]
crate = "libp2p-core"
version = "0.27.0"

[[target.peer]]
crate = "futures"
version = "0.3.12"
features = ["executor"]

[report]
issue_url = "https://github.com/libp2p/rust-libp2p/issues/1932"
issue_date = 2020-01-24
rustsec_url = "https://github.com/RustSec/advisory-db/pull/700"

[[bugs]]
analyzer = "UnsafeDataflow"
bug_class = "UninitExposure"
```
!*/
#![forbid(unsafe_code)]

use libp2p_core::upgrade::{InboundUpgrade, ProtocolName};
use libp2p_deflate::DeflateConfig;

use futures::executor;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, Error};
use futures::task::{Context, Poll};
use std::pin::Pin;

struct BrokenReader();

impl AsyncRead for BrokenReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        // Dump out uninitialized memory
        println!("{:?}", buf);
        assert!(buf[0] == 0);

        return Poll::Ready(Ok(buf.len()));
    }
}

impl AsyncWrite for BrokenReader {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        return Poll::Ready(Ok(buf.len()));
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Error>> {
        return Poll::Ready(Ok(()));
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Error>> {
        return Poll::Ready(Ok(()));
    }
}

fn main() {
    executor::block_on(async {
        let broken_reader = BrokenReader();
        let deflate_config = DeflateConfig::default();

        let mut deflate_output = deflate_config
            .upgrade_inbound(broken_reader, "/test/1".as_bytes())
            .await
            .unwrap();

        let mut buf = [1u8; 256];
        deflate_output.read_exact(&mut buf).await.unwrap();
    });
}
