
use std::io;
use std::fs;
use std::net;


pub struct Usb {}
pub struct Serial {}

#[derive(Debug)]
pub struct Network {
    _host: String,
    _port: u16,
    stream: net::TcpStream
}

impl Network {
    pub fn new(host: &str, port: u16) -> Network {
        let stream = net::TcpStream::connect((host, port)).unwrap();
        Network{
            _host: host.to_string(),
            _port: port,
            stream: stream
        }
    }
}

impl io::Write for Network {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}


#[derive(Debug)]
pub struct File<W> {
    _path: String,
    fobj: W
}

impl <W: io::Write> File<W> {
    pub fn new(path: &str) -> File<fs::File> {
        let fobj = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        File{_path: path.to_string(), fobj: fobj}
    }

    pub fn from(path: &str, fobj: W) -> File<W>{
        File{_path: path.to_string(), fobj: fobj}
    }
}

impl <W: io::Write> io::Write for File<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.fobj.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.fobj.flush()
    }
}
