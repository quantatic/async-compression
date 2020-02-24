use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{codec::Decode, util::PartialBuffer};
use bytes::{Buf, Bytes, BytesMut};
use futures_core::{ready, stream::Stream};
use pin_project_lite::pin_project;

const OUTPUT_BUFFER_SIZE: usize = 8_000;

#[derive(Debug)]
enum State {
    Reading,
    Writing,
    Flushing,
    Done,
}

pin_project! {
    #[derive(Debug)]
    pub struct Decoder<S, D: Decode> {
        #[pin]
        stream: S,
        decoder: D,
        state: State,
        input: Bytes,
        output: BytesMut,
    }
}

impl<S: Stream<Item = Result<Bytes>>, D: Decode> Decoder<S, D> {
    pub fn new(stream: S, decoder: D) -> Self {
        Self {
            stream,
            decoder,
            state: State::Reading,
            input: Bytes::new(),
            output: BytesMut::new(),
        }
    }

    pub fn get_ref(&self) -> &S {
        &self.stream
    }

    pub fn get_mut(&mut self) -> &mut S {
        &mut self.stream
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut S> {
        self.project().stream
    }

    pub fn into_inner(self) -> S {
        self.stream
    }
}

impl<S: Stream<Item = Result<Bytes>>, D: Decode> Stream for Decoder<S, D> {
    type Item = Result<Bytes>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes>>> {
        let this = self.project();

        let (mut stream, input, state, decoder) =
            (this.stream, this.input, this.state, this.decoder);

        let mut output = PartialBuffer::new(this.output);

        let result = (|| loop {
            let output_capacity = output.written().len() + OUTPUT_BUFFER_SIZE;
            output.get_mut().resize(output_capacity, 0);

            *state = match state {
                State::Reading => {
                    if let Some(chunk) = ready!(stream.as_mut().poll_next(cx)) {
                        *input = chunk?;
                        State::Writing
                    } else {
                        State::Flushing
                    }
                }

                State::Writing => {
                    if input.is_empty() {
                        State::Reading
                    } else {
                        let mut input = PartialBuffer::new(&mut *input);

                        let done = decoder.decode(&mut input, &mut output)?;

                        let input_len = input.written().len();
                        input.into_inner().advance(input_len);

                        if done {
                            State::Flushing
                        } else {
                            State::Writing
                        }
                    }
                }

                State::Flushing => {
                    if decoder.finish(&mut output)? {
                        State::Done
                    } else {
                        State::Flushing
                    }
                }

                State::Done => {
                    return Poll::Ready(None);
                }
            };
        })();

        match result {
            Poll::Ready(Some(Ok(_))) => unreachable!(),
            Poll::Ready(Some(Err(_))) => {
                *state = State::Done;
                result
            }
            Poll::Ready(None) | Poll::Pending => {
                if output.written().is_empty() {
                    result
                } else {
                    let output_len = output.written().len();
                    Poll::Ready(Some(Ok(output.into_inner().split_to(output_len).freeze())))
                }
            }
        }
    }
}
