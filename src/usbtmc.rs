use std::{error, fmt, path};

use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;

// IoCtl definitions taken from linux/usb/tmc.h
const USBTMC_IOC_NR: u8 = 91;
const USBTMC_IOCTL_API_VERSION: u8 = 16;

nix::ioctl_read!(usbtmc_get_api_version, USBTMC_IOC_NR, USBTMC_IOCTL_API_VERSION, u32);

#[derive(Debug)]
pub enum Error {
    NixError(nix::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NixError(err) => err.fmt(f)
        }
    }
}

impl error::Error for Error {
}

pub type Result<T> = std::result::Result<T, Error>;


pub struct UsbTmcDevice {
    fhndl: i32,
}

impl UsbTmcDevice {
    pub fn open(fpath: &path::Path) -> Result<Self> {
        match open(fpath, OFlag::O_RDWR, Mode::empty()) {
            nix::Result::Ok(fhndl) => Ok(Self{fhndl}),
            nix::Result::Err(err) => Err(Error::NixError(err)),
        }
    }

    pub fn get_api_version(&self) -> Result<u32> {
        let mut version: u32 = 0;
        unsafe {
            match usbtmc_get_api_version(self.fhndl, &mut version) {
                nix::Result::Ok(_) => Ok(version),
                nix::Result::Err(err) => Err(Error::NixError(err)),
            }
        }
    }

    pub fn write(&self, data: &[u8]) -> Result<usize> {
        match nix::unistd::write(self.fhndl, data) {
            nix::Result::Ok(nwritten) => Ok(nwritten),
            nix::Result::Err(err) => Err(Error::NixError(err)),
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        match nix::unistd::read(self.fhndl, buf) {
            nix::Result::Ok(nread) => Ok(nread),
            nix::Result::Err(err) => Err(Error::NixError(err)),
        }
    }

    fn close(&self) {
        nix::unistd::close(self.fhndl).unwrap();
    }
}

impl Drop for UsbTmcDevice {
    fn drop(&mut self) {
        self.close();
    }
}
