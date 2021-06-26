//! A generic CircularBuffer library
//!
//! Provies a circular buffer that is implemented as a heap-allocated generic of Optional objects.
//! Using Optional allows setting the contents (to None::<T>) when they are uninitialized or
//! unused


/// Implements a circular buffer generic
///
/// This is implemented with
pub struct CircularBuffer<T> {
    data: Box<[Option<T>]>,
    size: usize,
    start: usize,
    end: usize,
    is_full: bool,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut v: Vec<Option<T>> = Vec::with_capacity(capacity);
        v.resize_with(capacity, || None::<T>);
        let buffer = CircularBuffer{data: v.into_boxed_slice(),
            size: capacity,
            start: 0,
            end: 0,
            is_full: false};
        buffer
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full {
            Result::Err(Error::FullBuffer)
        } else {
            self.data[self.end] = Some(_element);
            self.end = (self.end + 1) % self.size;
            if self.start == self.end {
                self.is_full = true
            }
            Result::Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.start == self.end && !self.is_full {
            Result::Err(Error::EmptyBuffer)
        } else {
            self.is_full = false;
            if let Some(read_value) = self.data[self.start].take() {
                self.start = (self.start + 1) % self.size;
                Result::Ok(read_value)
            } else {
                // Shouldn't happen since index checking should take care of this
                self.start += 1;
                Result::Err(Error::EmptyBuffer)
            }
        }
    }

    pub fn clear(&mut self) {
        //unimplemented!("Clear the CircularBuffer.");
        while self.read().is_ok() {
            // Read values taking them one at a time
        }
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full {
            self.data[self.end] = Some(element);
            self.start = (self.start + 1) % self.size;
            self.end = self.start;
        } else {
            let _ = self.write(element); // Ignore the result; above check should suffice
        }
    }
}
