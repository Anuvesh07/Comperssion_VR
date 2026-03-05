use serde::{Deserialize, Serialize};

/// A bitstream writer for constructing compressed binary output bit-by-bit.
#[derive(Debug, Clone)]
pub struct BitstreamWriter {
    buffer: Vec<u8>,
    current_byte: u8,
    bit_position: u8,
}

impl BitstreamWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            current_byte: 0,
            bit_position: 0,
        }
    }

    /// Write a single bit (0 or 1).
    pub fn write_bit(&mut self, bit: bool) {
        if bit {
            self.current_byte |= 1 << (7 - self.bit_position);
        }
        self.bit_position += 1;

        if self.bit_position == 8 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bit_position = 0;
        }
    }

    /// Write multiple bits from a u64 value (MSB first).
    pub fn write_bits(&mut self, value: u64, num_bits: u8) {
        for i in (0..num_bits).rev() {
            self.write_bit((value >> i) & 1 == 1);
        }
    }

    /// Write a full byte.
    pub fn write_byte(&mut self, byte: u8) {
        self.write_bits(byte as u64, 8);
    }

    /// Write a string of '0' and '1' characters.
    pub fn write_bit_string(&mut self, bits: &str) {
        for ch in bits.chars() {
            match ch {
                '0' => self.write_bit(false),
                '1' => self.write_bit(true),
                _ => {} // ignore invalid characters
            }
        }
    }

    /// Flush any remaining bits (padding with zeros).
    pub fn flush(&mut self) {
        if self.bit_position > 0 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bit_position = 0;
        }
    }

    /// Get the total number of bits written.
    pub fn total_bits(&self) -> usize {
        self.buffer.len() * 8 + self.bit_position as usize
    }

    /// Consume the writer and return the byte buffer.
    pub fn into_bytes(mut self) -> Vec<u8> {
        self.flush();
        self.buffer
    }

    /// Get a reference to the current buffer.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl Default for BitstreamWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// A bitstream reader for reading compressed binary data bit-by-bit.
#[derive(Debug, Clone)]
pub struct BitstreamReader<'a> {
    data: &'a [u8],
    byte_position: usize,
    bit_position: u8,
}

impl<'a> BitstreamReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_position: 0,
            bit_position: 0,
        }
    }

    /// Read a single bit.
    pub fn read_bit(&mut self) -> Option<bool> {
        if self.byte_position >= self.data.len() {
            return None;
        }

        let bit = (self.data[self.byte_position] >> (7 - self.bit_position)) & 1 == 1;
        self.bit_position += 1;

        if self.bit_position == 8 {
            self.byte_position += 1;
            self.bit_position = 0;
        }

        Some(bit)
    }

    /// Read multiple bits as a u64 (MSB first).
    pub fn read_bits(&mut self, num_bits: u8) -> Option<u64> {
        let mut value: u64 = 0;
        for _ in 0..num_bits {
            let bit = self.read_bit()?;
            value = (value << 1) | if bit { 1 } else { 0 };
        }
        Some(value)
    }

    /// Read a full byte.
    pub fn read_byte(&mut self) -> Option<u8> {
        self.read_bits(8).map(|v| v as u8)
    }

    /// Get the total number of bits read so far.
    pub fn bits_read(&self) -> usize {
        self.byte_position * 8 + self.bit_position as usize
    }

    /// Check if there are more bits to read.
    pub fn has_more(&self) -> bool {
        self.byte_position < self.data.len()
    }
}

/// Visual representation of a bitstream for the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitstreamVisualization {
    /// The raw bytes.
    pub bytes: Vec<u8>,
    /// Binary string representation.
    pub binary_string: String,
    /// Total number of meaningful bits.
    pub total_bits: usize,
    /// Grouped display (e.g., groups of 8 bits).
    pub byte_groups: Vec<String>,
}

impl BitstreamVisualization {
    pub fn from_bytes(bytes: &[u8], total_bits: usize) -> Self {
        let mut binary_string = String::new();
        let mut byte_groups = Vec::new();
        let mut bits_shown = 0;

        for &byte in bytes {
            let mut group = String::new();
            for bit_pos in (0..8).rev() {
                if bits_shown >= total_bits {
                    break;
                }
                let bit = (byte >> bit_pos) & 1;
                let ch = if bit == 1 { '1' } else { '0' };
                binary_string.push(ch);
                group.push(ch);
                bits_shown += 1;
            }
            if !group.is_empty() {
                byte_groups.push(group);
            }
        }

        Self {
            bytes: bytes.to_vec(),
            binary_string,
            total_bits,
            byte_groups,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read_bits() {
        let mut writer = BitstreamWriter::new();
        writer.write_bit(true);
        writer.write_bit(false);
        writer.write_bit(true);
        writer.write_bit(true);
        writer.write_bit(false);
        writer.write_bit(false);
        writer.write_bit(true);
        writer.write_bit(false);

        let bytes = writer.into_bytes();
        assert_eq!(bytes, vec![0b10110010]);

        let mut reader = BitstreamReader::new(&bytes);
        assert_eq!(reader.read_bit(), Some(true));
        assert_eq!(reader.read_bit(), Some(false));
        assert_eq!(reader.read_bit(), Some(true));
        assert_eq!(reader.read_bit(), Some(true));
        assert_eq!(reader.read_bit(), Some(false));
        assert_eq!(reader.read_bit(), Some(false));
        assert_eq!(reader.read_bit(), Some(true));
        assert_eq!(reader.read_bit(), Some(false));
    }

    #[test]
    fn test_write_bits_multi() {
        let mut writer = BitstreamWriter::new();
        writer.write_bits(0b11010, 5);
        writer.write_bits(0b110, 3);

        let bytes = writer.into_bytes();
        assert_eq!(bytes, vec![0b11010110]);
    }

    #[test]
    fn test_write_bit_string() {
        let mut writer = BitstreamWriter::new();
        writer.write_bit_string("10110010");
        let bytes = writer.into_bytes();
        assert_eq!(bytes, vec![0b10110010]);
    }

    #[test]
    fn test_partial_byte_flush() {
        let mut writer = BitstreamWriter::new();
        writer.write_bit(true);
        writer.write_bit(true);
        writer.write_bit(false);

        assert_eq!(writer.total_bits(), 3);
        let bytes = writer.into_bytes();
        assert_eq!(bytes, vec![0b11000000]);
    }

    #[test]
    fn test_visualization() {
        let bytes = vec![0b10110010, 0b11110000];
        let vis = BitstreamVisualization::from_bytes(&bytes, 16);
        assert_eq!(vis.binary_string, "1011001011110000");
        assert_eq!(vis.byte_groups.len(), 2);
    }

    #[test]
    fn test_visualization_partial() {
        let bytes = vec![0b10110010, 0b11100000];
        let vis = BitstreamVisualization::from_bytes(&bytes, 11);
        assert_eq!(vis.binary_string, "10110010111");
        assert_eq!(vis.total_bits, 11);
    }
}
