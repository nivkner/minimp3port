use super::ffi;

pub struct Bits<'a> {
    pub data: &'a [u8],
    pub position: usize,
}

impl<'a> Bits<'a> {
    pub fn new_with_pos(data: &'a [u8], position: usize) -> Self {
        Bits { data, position }
    }

    pub fn new(data: &'a [u8]) -> Self {
        Bits::new_with_pos(data, 0)
    }

    pub fn limit(&self) -> usize {
        self.data.len() * 8
    }

    // use when a bs_t is needed to preserve the original lifetime
    // the caller must ensure that the bs_t does not outlive self
    pub unsafe fn bs_copy(&self) -> ffi::bs_t {
        ffi::bs_t {
            buf: self.data.as_ptr(),
            pos: self.position as _,
            limit: self.limit() as _,
        }
    }

    pub fn get_bits(&mut self, amount: u32) -> u32 {
        let s = self.position & 7;
        let mut idx = self.position as usize >> 3;
        self.position += amount as usize;
        if self.position > self.limit() {
            return 0;
        }

        let mut next: u32 = u32::from(self.data[idx]) & (255 >> s);
        idx += 1;
        let mut shl: i32 = amount as i32 + s as i32;

        let mut cache: u32 = 0;
        loop {
            shl -= 8;
            if shl <= 0 {
                break;
            }
            cache |= next << shl;
            next = self.data[idx].into();
            idx += 1;
        }

        (cache | (next >> -shl)) as _
    }
}
