use std::fs;
use std::io;
use std::net;
use std::path;

use rusb::Direction;
use rusb::TransferType;
use rusb::UsbContext;
use rusb::{Context, DeviceHandle};

pub struct Usb {
    _vendor_id: u16,
    _product_id: u16,
    connection: DeviceHandle<Context>,
    endpoint: u8,
}

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

impl Usb {
    /// Create a new USB device.
    /// # Example
    /// ```no_run
    /// use std::io;
    /// use escposify::device::Usb;
    /// use escposify::printer::Printer;
    ///
    /// let product_id = 0xa700;
    /// let vendor_id = 0x0525;
    /// let usb = Usb::new(vendor_id, product_id).unwrap();
    /// let mut printer = Printer::new(usb, None, None);
    /// ```
    pub fn new(vendor_id: u16, product_id: u16) -> io::Result<Usb> {
        let context = Context::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let devices = context
            .devices()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        for device in devices.iter() {
            let device_desc = device
                .device_descriptor()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            if device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id {
                let config_descriptor = device
                    .active_config_descriptor()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                let endpoint = config_descriptor
                    .interfaces()
                    .flat_map(|interface| interface.descriptors())
                    .flat_map(|descriptor| descriptor.endpoint_descriptors())
                    .find_map(|endpoint| {
                        if let (TransferType::Bulk, Direction::Out) =
                            (endpoint.transfer_type(), endpoint.direction())
                        {
                            Some(endpoint.number())
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::Other, "No suitable endpoint found")
                    })?;

                match device.open() {
                    Ok(mut dvc) => {
                        if let Ok(active) = dvc.kernel_driver_active(0) {
                            if active {
                                if let Err(e) = dvc.detach_kernel_driver(0) {
                                    return Err(io::Error::new(io::ErrorKind::Other, e));
                                }
                            }
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                "Error checking kernel driver",
                            ));
                        };

                        return dvc
                            .claim_interface(0)
                            .map(|_| Usb {
                                _vendor_id: vendor_id,
                                _product_id: product_id,
                                connection: dvc,
                                endpoint,
                            })
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e));
                    }
                    Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Device busy")),
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::Other, "USB not found"))
    }
}

impl io::Write for Usb {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self
            .connection
            .write_bulk(self.endpoint, buf, std::time::Duration::from_secs(5))
        {
            Ok(_) => Ok(buf.len()),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
