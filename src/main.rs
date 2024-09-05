use std::net::TcpStream;
use std::io::{self, Read};

pub trait Future {
    type Output;
    fn poll(&mut self) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}

struct Read<'buf, 'read, R> {
    reader: &'read mut R,
    buf: &'buf mut [u8],
}

enum CaptureFuture<'x> {
    Init {
        x: &'x str
    },
    Done,
}

enum TcpRead<'buf, 'read> {
    Init {
        buf: Vec<u8>
    },

    Reader {
        buf: Vec<u8>,
        reader: TcpStream,
    },

    Reading {
        buf: Vec<u8>,
        reader: TcpStream,
        reading: Read<'buf, 'read, TcpStream>,
    },

    Done,
}

impl<'buf, 'read> Future for TcpRead<'buf, 'read> {
    type Output = ();
    fn poll(&mut self) -> Poll<Self::Output> {
        let this = std::mem::replace(self, Self::Done);
        let new = match this {
            Self::Init { buf } => {
                Self::Reader { buf, reader: TcpStream }
            },
            Self::Reader { mut buf, mut reader } => {
                let reading = reader.read(&mut buf);
                Self::Reading { buf, reader, reading }
            },

            Self::Reading { buf, reader, mut reading } => {
                match reading.poll() {
                    Poll::Pending => Reading { buf, reader, reading },
                    Poll::Ready(n) => {
                        let n = n.unwrap();
                        println!("Read {:?}", &buf[..n]);
                        return Poll::Ready(());
                    }
                }
            },
            Self::Done => panic!("I am done!"),
        };
        *self = new;
        Poll::Pending
    }
}

impl Future for CaptureFuture<'_> {
    type Output = ();

    fn poll(&mut self) -> Poll<Self::Output> {
        match self {
            Self::Init {x} => {
                println!("{x}");
                *self = Self::Done;
                Poll::Ready(())
            },
            Self::Done => panic!("No more polling!"),
        }
    }
}

#[derive(Debug)]
enum MyFuture {
    Init,
    Step1(usize),
    Done,
}

impl Future for MyFuture {
    type Output = usize;
    fn poll(&mut self) -> Poll<Self::Output> {
        let this = std::mem::replace(self, Self::Done);

        let new = match this {
            Self::Init => Self::Step1(6),
            Self::Step1(n) => return Poll::Ready(n * 7),
            Self::Done => panic!("Please stop the polling!"),
        };

        *self = new;
        Poll::Pending
    }
}


fn main() {
    let mut fut = MyFuture::Init;
    let n = loop {
        println!("Polling - {fut:?}");
        match fut.poll() {
            Poll::Ready(n) => break n,
            Poll::Pending => println!("pending"),
        }
    };
    println!("Ready - {n:?}");

    
}
