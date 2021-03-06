use bytes::{BigEndian, ByteOrder};
use std::env;
use std::io::{self, Write};
#[cfg(windows)]
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::net::UnixStream;

pub struct Session<T>
where
    T: Write,
{
    transport: T,
}

#[cfg(unix)]
impl Session<UnixStream> {
    pub fn connect() -> Result<Session<UnixStream>, io::Error> {
        UnixStream::connect(env::var("LOGGESTD_SOCKET").unwrap_or_else(|_| "/run/loggestd.sock".into()))
            .map(|transport| Session { transport })
    }
}

#[cfg(windows)]
impl Session<TcpStream> {
    pub fn connect() -> Result<Session<TcpStream>, io::Error> {
        TcpStream::connect(env::var("LOGGESTD_SOCKET").unwrap_or_else(|_| "127.0.0.1:1099".into()))
            .map(|transport| Session { transport })
    }
}

impl<T> Session<T>
where
    T: Write,
{
    pub fn establish(mut self, filename: &str) -> Result<EstablishedSession<T>, io::Error> {
        let filename = filename.as_bytes();
        let mut buffer = [0; 2];

        BigEndian::write_u16(&mut buffer, filename.len() as u16);
        self.transport.write_all(&buffer)?;
        self.transport.write_all(filename)?;

        Ok(EstablishedSession {
            transport: self.transport,
        })
    }
}

pub struct EstablishedSession<T>
where
    T: Write,
{
    transport: T,
}

impl<T> Write for EstablishedSession<T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.transport.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.transport.flush()
    }
}
