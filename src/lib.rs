#![feature(local_waker)]

mod executor;
use crate::executor::BoxFuture;
pub use crate::executor::NetExecutor;
use crate::executor::NetTask;

mod r#async;
pub use crate::r#async::tcp_listener::AsyncTcpListener;
pub use crate::r#async::tcp_stream::AsyncTcpStream;

mod futures;
use crate::futures::AsyncReadLineFuture;
use crate::futures::AsyncTcpStreamFuture;
use crate::futures::AsyncWriteAllFuture;
