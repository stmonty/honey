use std::net::TcpStream;
use std::io::Read;

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>) -> Poll<Self::Output>;
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
