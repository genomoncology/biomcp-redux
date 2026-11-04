//! A signal interrupts an attempt, not the caller's read or deadline measurement.
use std::io::{self, Read};

pub(crate) fn read(reader: &mut impl Read, buffer: &mut [u8]) -> io::Result<usize> {
    loop {
        match reader.read(buffer) {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            result => return result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scripted(std::vec::IntoIter<io::Result<usize>>);
    impl Read for Scripted {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            self.0.next().expect("unexpected extra read")
        }
    }

    #[test]
    fn interrupted_read_continues_to_eof() {
        let mut reader = Scripted(vec![Err(io::ErrorKind::Interrupted.into()), Ok(0)].into_iter());
        assert_eq!(read(&mut reader, &mut [0]).unwrap(), 0);
        assert!(reader.0.next().is_none());
    }

    #[test]
    fn other_read_errors_are_returned_without_retry() {
        for kind in [
            io::ErrorKind::TimedOut,
            io::ErrorKind::WouldBlock,
            io::ErrorKind::ConnectionReset,
            io::ErrorKind::BrokenPipe,
        ] {
            let mut reader = Scripted(vec![Err(kind.into())].into_iter());
            assert_eq!(read(&mut reader, &mut [0]).unwrap_err().kind(), kind);
        }
    }
}
