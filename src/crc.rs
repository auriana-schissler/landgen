pub struct Crc32 {
    table: [u32; 256],
    value: u32,
}

const CRC32_INITIAL: u32 = 0xedb88320;

impl Crc32 {
    pub fn new() -> Crc32 {
        let mut c = Crc32 {
            table: [0; 256],
            value: 0xffffffff,
        };
        for i in 0..256 {
            let mut v = i as u32;
            for _ in 0..8 {
                v = if v & 1 != 0 {
                    CRC32_INITIAL ^ (v >> 1)
                } else {
                    v >> 1
                }
            }
            c.table[i] = v;
        }
        c
    }

    pub fn start(&mut self) {
        self.value = 0xffffffff;
    }

    pub fn update(&mut self, buf: &[u8]) {
        for &i in buf {
            self.value =
                self.table[((self.value ^ (i as u32)) & 0xff) as usize] ^ (self.value >> 8);
        }
    }

    pub fn finalize(&mut self) -> u32 {
        self.value ^ 0xffffffff_u32
    }

    pub fn crc(&mut self, buf: &[u8]) -> u32 {
        self.start();
        self.update(buf);
        self.finalize()
    }
}

#[test]
fn test_crc() {
    assert_eq!(Crc32::new().crc(b"IEND"), u32::from_be_bytes([0xae, 0x42, 0x60, 0x82]));
    assert_eq!(Crc32::new().crc(&[
        0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x06, 0x6a, 0x00, 0x00, 0x04, 0x47, 0x08, 0x02, 0x00, 0x00, 0x00
    ]), u32::from_be_bytes([0x7c, 0x8b, 0xab, 0x78]));
}
