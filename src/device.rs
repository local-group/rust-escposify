use std::fs;
use std::io;
use std::net;
use std::path;

pub struct Usb {}
pub struct Serial {}

#[derive(Debug)]
pub struct Network {
    _host: String,
    _port: u16,
    stream: net::TcpStream,
}

impl Network {
    pub fn new(host: &str, port: u16) -> io::Result<Network> {
        let stream = net::TcpStream::connect((host, port))?;
        Ok(Network {
            _host: host.to_string(),
            _port: port,
            stream,
        })
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

/// File device that can be written to.

#[derive(Debug)]
pub struct File<W> {
    fobj: W,
}

impl<W: io::Write> File<W> {
    pub fn from_path<P: AsRef<path::Path> + ToString>(path: P) -> io::Result<File<fs::File>> {
        let fobj = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)?;
        Ok(File { fobj })
    }

    /// Create a device::File from a [std::io::Write].
    /// # Example
    /// ```rust
    /// use std::fs::File;
    /// use tempfile::NamedTempFileOptions;
    ///
    /// let tempf = NamedTempFileOptions::new().create().unwrap();
    /// let fobj = File::options().append(true).open(tempf.path()).unwrap();
    /// let file = escposify::device::File::from(fobj);
    /// ```
    pub fn from(fobj: W) -> File<W> {
        File { fobj }
    }
}

impl<W: io::Write> io::Write for File<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.fobj.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.fobj.flush()
    }
}
