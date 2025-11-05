#![warn(rust_2018_idioms)]
#![cfg(all(feature = "time", not(target_os = "wasi")))] // Wasi does not support panic recovery
#![cfg(panic = "unwind")]

use std::error::Error;
use tokio::time::Duration;
use tokio_stream::{self as stream, StreamExt};

#[tokio::test]
async fn stream_chunks_remainder() -> Result<(), Box<dyn Error>> {
    let inner = stream::iter([1, 2, 3, 4]).throttle(Duration::from_millis(2));
    tokio::pin!(inner);

    let chunked = (&mut inner).chunks_timeout(4, Duration::from_millis(8));
    tokio::pin!(chunked);

    tokio::select! {
        Some(_chunk) = chunked.next() => {}
        _ = tokio::time::sleep(Duration::from_millis(4)) => {}
    }

    assert_eq!(chunked.into_remainder(), vec![1, 2]);
    assert_eq!(inner.next().await, Some(3));
    Ok(())
}
