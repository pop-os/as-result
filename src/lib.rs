use std::{io, process::ExitStatus};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt as _;

/// Converts the type into a `std::io::Result<()>` by reference
pub trait AsResult<T, E> {
    /// Converts the type into a `std::io::Result<()>` by reference
    fn as_result(&self) -> Result<T, E>;
}

/// Converts the type into a `std::io::Result<()>`
pub trait IntoResult<T, E>: AsResult<T, E> + Sized {
    /// Converts the type into a `std::io::Result<()>`
    fn into_result(self) -> Result<T, E> {
        self.as_result()
    }
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

impl IntoResult<(), io::Error> for ExitStatus {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn command() {
        Command::new("/usr/bin/echo")
            .arg("hello world")
            .status()
            .and_then(IntoResult::into_result)
            .unwrap();

        Command::new("/usr/bin/echo")
            .arg("hello world")
            .status()
            .unwrap()
            .into_result()
            .unwrap()
    }
}
