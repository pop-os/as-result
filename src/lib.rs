use std::{
    io,
    process::{ExitStatus, Output},
};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt as _;

/// Converts the type into a `std::io::Result<()>` by reference
pub trait AsResult<T, E> {
    /// Converts the type into a `std::io::Result<()>` by reference
    fn as_result(&self) -> Result<T, E>;
}

/// Converts the type into a `std::io::Result<()>`
pub trait IntoResult<T, E> {
    /// Converts the type into a `std::io::Result<()>`
    fn into_result(self) -> Result<T, E>;
}

/// Maps a result into another result
pub trait MapResult<T, E> {
    /// Maps a result into another result
    fn map_result(self) -> Result<T, E>;
}

impl AsResult<(), io::Error> for ExitStatus {
    fn as_result(&self) -> io::Result<()> {
        Err(if self.success() {
            return Ok(());
        } else if let Some(127) = self.code() {
            io::Error::new(io::ErrorKind::NotFound, "command was not found")
        } else {
            #[cfg(unix)]
            {
                if let Some(signal) = self.signal() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("terminated with signal {}", signal),
                    ));
                }
            }

            io::Error::new(io::ErrorKind::Other, format!("status is unknown: {}", self))
        })
    }
}

impl IntoResult<(), io::Error> for ExitStatus {
    fn into_result(self) -> io::Result<()> {
        self.as_result()
    }
}

impl IntoResult<Output, io::Error> for Output {
    fn into_result(self) -> io::Result<Output> {
        self.status.as_result().map(|_| self)
    }
}

impl MapResult<(), io::Error> for io::Result<ExitStatus> {
    fn map_result(self) -> io::Result<()> {
        self.and_then(IntoResult::into_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn command() {
        Command::new("/bin/echo")
            .arg("hello world")
            .status()
            .and_then(IntoResult::into_result)
            .unwrap();

        Command::new("/bin/echo")
            .arg("hello world")
            .status()
            .unwrap()
            .into_result()
            .unwrap();

        Command::new("/bin/echo")
            .arg("hello world")
            .status()
            .map_result()
            .unwrap()
    }
}
