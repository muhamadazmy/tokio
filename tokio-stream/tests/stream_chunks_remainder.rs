#![warn(rust_2018_idioms)]
#![cfg(all(feature = "time", not(target_os = "wasi")))] // Wasi does not support panic recovery
#![cfg(panic = "unwind")]

use futures::FutureExt;
use std::error::Error;
use tokio::time::Duration;
use tokio_stream::{self as stream, StreamExt};

#[tokio::test]
async fn stream_chunks_remainder() -> Result<(), Box<dyn Error>> {
    let inner = stream::iter([1, 2, 3, 4]).chain(stream::pending());
    tokio::pin!(inner);

    let chunked = (&mut inner).chunks_timeout(10, Duration::from_millis(20));
    tokio::pin!(chunked);

    tokio::select! {
        Some(_chunk) = chunked.next() => {}
        _ = tokio::time::sleep(Duration::from_millis(3)) => {}
    }

    assert_eq!(chunked.into_remainder(), vec![1, 2, 3, 4]);
    assert_eq!(inner.next().now_or_never(), None);
    Ok(())
}
