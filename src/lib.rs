#![crate_name = "circular_buffer"]
//! A generic CircularBuffer library
//!
//! Provides a circular buffer that is implemented as a heap-allocated generic of Optional objects.
//! Using Optional allows setting the contents (to None::<T>) when they are unused.


/// Implements a circular buffer generic
///
/// This is implemented using a Boxed slice of options
pub struct CircularBuffer<T> {
    /// The data held in the buffer
    data: Box<[Option<T>]>,
    /// The size of the buffer
    size: usize,
    /// The start index
    start: usize,
    /// The end index
    end: usize,
}

#[derive(Debug, PartialEq)]
/// An Error type for handling Error cases for CircularBuffer structures
pub enum Error {
    /// An error indicating the buffer is empty (e.g. on a read)
    EmptyBuffer,
    /// An error indicating the buffer is full (e.g. on a write)
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    /// Returns a new CircularBuffer with the specified capacity
    ///
    /// # Arguments
    ///
    /// * `capacity` The capacity of the buffer
    ///
    /// # Example
    /// ```
    /// use circular_buffer::CircularBuffer;
    /// buffer = CircularBuffer::<u32>::new(10); // Make a buffer with 10-number capacity
    /// ```
    pub fn new(capacity: usize) -> Self {
        let mut v: Vec<Option<T>> = Vec::with_capacity(capacity);
        v.resize_with(capacity, || None::<T>);
        let buffer = CircularBuffer{data: v.into_boxed_slice(),
            size: capacity,
            start: 0,
            end: 0,
        };
        buffer
    }

    /// Write a value to the circular buffer
    ///
    /// # Arguments
    /// * `element` The element to be added to the buffer
    ///
    /// # Returns
    /// * `Ok(())` for success
    /// * `Error::FullBuffer` if buffer is full
    ///
    /// # Examples
    /// ```
    /// use circular_buffer::CircularBuffer;
    /// let mut buffer = CircularBuffer::<u32>::new(1);
    /// buffer.write(5);
    /// assert_eq!(5, buffer.read());
    /// ```
    /// ```
    /// use circular_buffer::{CircularBuffer, Error};
    /// let mut buffer = CircularBuffer::<u32>::new(1);
    /// buffer.write(5);
    /// assert_eq!(Result::Err(Error::FullBuffer), buffer.write(6));
    /// ```
    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.start == self.end && self.data[self.start].is_some() {
            Result::Err(Error::FullBuffer)
        } else {
            self.data[self.end] = Some(element);
            self.end = (self.end + 1) % self.size;
            Result::Ok(())
        }
    }

    /// Read a value from the circular buffer
    ///
    /// # Returns
    /// * `Ok(T)` The value is successfully read
    /// * `Err(Error::EmptyBuffer)` The buffer was empty
    pub fn read(&mut self) -> Result<T, Error> {
        if self.data[self.start].is_none() {
            Result::Err(Error::EmptyBuffer)
        } else {
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

    /// Clears the buffer
    ///
    /// All values are read from the buffer and discarded leaving an empty circular buffer.
    pub fn clear(&mut self) {
        //unimplemented!("Clear the CircularBuffer.");
        while self.read().is_ok() {
            // Read values taking them one at a time
        }
    }

    /// Forces writing a value, even when the buffer is full
    ///
    /// For a non-full buffer, this is equivalent to a normal write.  If the buffer is full, this is
    /// equivalent to a read and a write.
    pub fn overwrite(&mut self, element: T) {
        if self.start == self.end && self.data[self.start].is_some() {
            self.data[self.end] = Some(element);
            self.start = (self.start + 1) % self.size;
            self.end = self.start;
        } else {
            let _ = self.write(element); // Ignore the result; above check should suffice
        }
    }
}
