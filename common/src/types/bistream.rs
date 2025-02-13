use crate::protocol::{Frame, MessageCodec};
use anyhow::Result;
use futures::{Sink, SinkExt, Stream, StreamExt};
use quinn::{Connection, RecvStream, SendStream, StreamId};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_util::codec::{FramedRead, FramedWrite};

pub type ReadStream = FramedRead<RecvStream, MessageCodec>;
pub type WriteStream = FramedWrite<SendStream, MessageCodec>;

pub struct BiStream {
    write: WriteStream,
    read: ReadStream,
}

impl BiStream {
    pub async fn try_from_connection(connection: &Connection) -> Result<Self> {
        let stream = connection.open_bi().await?;
        Ok(Self::from(stream))
    }

    pub fn get_recv_stream_id(&self) -> StreamId {
        self.read.get_ref().id()
    }

    pub fn get_send_stream_id(&self) -> StreamId {
        self.write.get_ref().id()
    }

    pub async fn finish(&mut self) -> Result<()> {
        self.write.get_mut().finish().await?;
        Ok(())
    }
}

impl From<(SendStream, RecvStream)> for BiStream {
    fn from((send, recv): (SendStream, RecvStream)) -> Self {
        let write = FramedWrite::new(send, MessageCodec);
        let read = FramedRead::new(recv, MessageCodec);

        Self { write, read }
    }
}

impl Sink<Frame> for BiStream {
    type Error = anyhow::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.write.poll_ready_unpin(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: Frame) -> Result<(), Self::Error> {
        self.write.start_send_unpin(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.write.poll_flush_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.write.poll_close_unpin(cx)
    }
}

impl Stream for BiStream {
    type Item = Result<Frame>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.read.poll_next_unpin(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.read.size_hint()
    }
}
