//! Consistent-overhead byte stuffing
//!
//! Source: <http://www.stuartcheshire.org/papers/COBSforToN.pdf>

use canadensis_core::OutOfMemoryError;

/// Encodes a sequence of bytes and writes the encoded form to a destination
///
/// This function returns the number of bytes that were written to the destination.
///
/// This function returns an error if the destination is not long enough to hold the complete
/// encoded form.
///
#[cfg(test)]
pub fn escape(source: &[u8], destination: &mut [u8]) -> Result<usize, OutOfMemoryError> {
    escape_from_iter(source.iter().copied(), destination)
}

/// Encodes a sequence of bytes from an iterator and writes the encoded form to a destination
///
/// This function returns the number of bytes that were written to the destination.
///
/// This function returns an error if the destination is not long enough to hold the complete
/// encoded form.
///
pub fn escape_from_iter<I>(source: I, destination: &mut [u8]) -> Result<usize, OutOfMemoryError>
where
    I: IntoIterator<Item = u8>,
{
    // Index in destination to write the next data byte
    let mut dest_current = 1usize;
    // Index in destination to write the next code byte
    let mut dest_code = 0usize;
    let mut code: u8 = 0x1;

    for byte in source {
        if byte == 0 {
            // Write the code (finish block operation)
            let code_entry = destination.get_mut(dest_code).ok_or(OutOfMemoryError)?;
            *code_entry = code;
            code = 0x1;
            // Advance the place where the next code will be stored
            dest_code = dest_current;
            dest_current += 1;
        } else {
            if code == 0xff {
                // No zeros in 254 bytes (finish block operation)
                let entry = destination.get_mut(dest_code).ok_or(OutOfMemoryError)?;
                *entry = code;
                code = 0x1;
                dest_code = dest_current;
                dest_current += 1;
            }
            // Copy a normal, non-zero byte
            let entry = destination.get_mut(dest_current).ok_or(OutOfMemoryError)?;
            *entry = byte;
            dest_current += 1;
            code += 1;
        }
    }
    // Handle the logical zero at the end of the input, if one is not there already
    let code_entry = destination.get_mut(dest_code).ok_or(OutOfMemoryError)?;
    *code_entry = code;
    // Return the index of the last byte written in the destination
    Ok(dest_current)
}

/// Decodes a sequence of bytes and writes the decoded form to a destination
///
/// The minimum length of the destination is one byte less than the length of the source.
///
/// This function returns the number of bytes that were written to the destination.
#[cfg(test)]
pub fn unescape(source: &[u8], destination: &mut [u8]) -> Result<usize, DecodeError> {
    println!("Decode {:?}", source);
    let dest_len = destination.len();
    let mut src_iter = source.iter();
    let mut dest_iter = destination.iter_mut();
    while let Some(&code) = src_iter.next() {
        if code == 0 {
            // Not allowed
            return Err(DecodeError);
        }
        // Copy code bytes
        let copy_count = usize::from(code - 1);
        let src_sub = src_iter.as_slice();
        if src_sub.len() < copy_count {
            println!(
                "src_sub.len() {} < copy_count {}",
                src_sub.len(),
                copy_count
            );
            // Not enough source bytes to copy
            return Err(DecodeError);
        }
        let (src_read, src_others) = src_sub.split_at(copy_count);
        // Advance src_iter beyond the bytes we're about to copy
        src_iter = src_others.iter();
        let dst_sub = dest_iter.into_slice();
        if dst_sub.len() < copy_count {
            println!(
                "dst_sub.len() {} < copy_count {}",
                dst_sub.len(),
                copy_count
            );
            // Not enough destination space to copy
            return Err(DecodeError);
        }
        let (dst_write, dst_others) = dst_sub.split_at_mut(copy_count);
        dest_iter = dst_others.iter_mut();
        // Actually do the copy
        dst_write.copy_from_slice(src_read);

        if code < 0xff {
            // That was a segment that ended with a zero
            // If this is just the implicit zero at the end, don't add it
            if let Some(entry) = dest_iter.next() {
                *entry = 0x0;
            }
        }
    }

    // Number of bytes written = destination length - (remaining part of destination) length
    Ok(dest_len - dest_iter.as_slice().len())
}

/// A streaming unescaper that accepts one character at a time
pub struct Unescaper {
    /// The number of bytes to copy directly from the input to the output,
    /// instead of checking for a code
    bytes_to_copy: u8,
    /// If an extra 0 byte should be produced when bytes_to_copy reaches 0
    pending_zero: bool,
}

impl Unescaper {
    pub fn new() -> Self {
        Unescaper {
            bytes_to_copy: 0,
            pending_zero: false,
        }
    }

    pub fn accept(&mut self, byte: u8) -> Result<Option<u8>, DecodeZeroError> {
        if byte == 0 {
            Err(DecodeZeroError)
        } else {
            if self.bytes_to_copy == 0 {
                self.bytes_to_copy = byte - 1;
                let result = if self.pending_zero { Some(0) } else { None };
                self.pending_zero = byte < 0xff;
                Ok(result)
            } else {
                // Pass through
                self.bytes_to_copy -= 1;
                Ok(Some(byte))
            }
        }
    }
}

/// Determines the worst-case encoded size for a sequence of bytes of the provided size
pub fn escaped_size(raw_size: usize) -> usize {
    // COBS adds up to 1 byte for every 254 bytes of input
    if raw_size == 0 {
        1
    } else {
        // Divide and round up
        let overhead = (raw_size + 253) / 254;
        raw_size + overhead
    }
}

#[derive(Debug)]
pub struct DecodeError;

/// An error that occurs if the unescaper encounters a zero byte
#[derive(Debug)]
pub struct DecodeZeroError;

#[cfg(test)]
mod tests {
    use super::{escape, unescape};
    use crate::cobs::Unescaper;

    /// Test cases: (original, escaped)
    const TEST_CASES: &[(&[u8], &[u8])] = &[
        (&[], &[1]),
        (&[1], &[2, 1]),
        (&[1, 2], &[3, 1, 2]),
        (&[1, 0], &[2, 1, 1]),
        (
            &[
                0x45, 0x00, 0x00, 0x2c, 0x4c, 0x79, 0x00, 0x00, 0x40, 0x06, 0x4f, 0x37,
            ],
            &[
                0x02, 0x45, 0x01, 0x04, 0x2c, 0x4c, 0x79, 0x01, 0x05, 0x40, 0x06, 0x4f, 0x37,
            ],
        ),
        (
            &[1; 254],
            // 255 followed by 254 1s
            &[
                255, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1,
            ],
        ),
    ];

    #[test]
    fn test_encode() {
        for &(input, expected) in TEST_CASES {
            let mut buffer = vec![0u8; expected.len()];
            let encoded_bytes = escape(input, &mut buffer).unwrap();
            assert_eq!(encoded_bytes, expected.len());
            let buffer_used = &buffer[..expected.len()];
            assert_eq!(buffer_used, expected);
        }
    }

    #[test]
    fn test_decode() {
        for &(expected, input) in TEST_CASES {
            let mut buffer = vec![0u8; expected.len()];
            let decoded_bytes = unescape(input, &mut buffer).unwrap();
            assert_eq!(decoded_bytes, expected.len());
            let buffer_used = &buffer[..decoded_bytes];
            assert_eq!(expected, buffer_used);
        }
    }

    #[test]
    fn test_streaming_decode() {
        for &(expected, input) in TEST_CASES {
            let mut buffer = Vec::new();
            let mut unescaper = Unescaper::new();
            for &byte in input {
                if let Some(byte_out) = unescaper.accept(byte).unwrap() {
                    buffer.push(byte_out);
                }
            }
            assert_eq!(expected, buffer);
        }
    }
}
