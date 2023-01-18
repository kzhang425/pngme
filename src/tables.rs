// Implementation of the crc algorithm for PNG files
// First we can speed up calculation by doing the calculations first for all kinds of 8-bit chunks. This means that we can do byte-wise calculation instead.
// We will first generate the table

// There are 3 major kinds of CRC code calculations.
const TABLE_SIZE: usize = 256;
pub enum CrcType {
    // Enum contains the polynomial in the respective data type
    Crc8(u8),
    Crc16(u16),
    Crc32(u32),
}

impl From<u8> for CrcType {
    fn from(value: u8) -> Self {
        Self::Crc8(value)
    }
}

impl From<u16> for CrcType {
    fn from(value: u16) -> Self {
        Self::Crc16(value)
    }
}

impl From<u32> for CrcType {
    fn from(value: u32) -> Self {
        Self::Crc32(value)
    }
}

pub enum LookupTable {
    Crc8([u8; TABLE_SIZE]),
    Crc16([u16; TABLE_SIZE]),
    Crc32([u32; TABLE_SIZE]),
}

impl CrcType {
    pub fn make_table(&self) -> LookupTable
    {
        match self {
            &CrcType::Crc8(poly) => LookupTable::Crc8(make_table_u8(poly)),
            &CrcType::Crc16(poly) => LookupTable::Crc16(make_table_u16(poly)),
            &CrcType::Crc32(poly) => LookupTable::Crc32(make_table_u32(poly)),
        }
    }
}

fn make_table_u8(generator: u8) -> [u8; 256] {
    // First allocate a block of memory for the table to begin with
    let mut table:[u8; 256] = [0; 256];

    for divident in 0..255 {
        let mut cur_byte = divident;
        for _ in 0..7 {
            if (cur_byte & 0x80) != 0 {
                cur_byte <<= 1;
                cur_byte = cur_byte ^ generator;
            } else {
                cur_byte <<= 1;
            }
        }
        table[divident as usize] = cur_byte;
    }
    table
}

fn make_table_u16(generator: u16) -> [u16; 256] {
    // First allocate a block of memory for the table to begin with
    let mut table:[u16; 256] = [0; 256];

    for divident in 0..255 {
        let mut cur_byte = divident << 8;
        for _ in 0..7 {
            if (cur_byte & 0x8000) != 0 {
                cur_byte <<= 1;
                cur_byte = cur_byte ^ generator;
            } else {
                cur_byte <<= 1;
            }
        }
        table[divident as usize] = cur_byte;
    }
    table
}

fn make_table_u32(generator: u32) -> [u32; 256] {
    // First allocate a block of memory for the table to begin with
    let mut table:[u32; 256] = [0; 256];

    for divident in 0..255 {
        let mut cur_byte = divident << 24;
        for _ in 0..7 {
            if (cur_byte & 0x80000000) != 0 {
                cur_byte <<= 1;
                cur_byte = cur_byte ^ generator;
            } else {
                cur_byte <<= 1;
            }
        }
        table[divident as usize] = cur_byte;
    }
    table
}