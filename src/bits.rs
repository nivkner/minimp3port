use super::ffi;

#[derive(PartialEq, Debug)]
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

        let mut next: u32 = self.data[idx] as u32 & (255 >> s);
        idx += 1;
        let mut shl: i32 = amount as i32 + s as i32;

        let mut cache: u32 = 0;
        loop {
            shl -= 8;
            if shl <= 0 {
                break;
            }
            cache |= next << shl;
            next = self.data[idx] as u32;
            idx += 1;
        }

        return (cache | (next >> -shl)) as _;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;
    use std::vec::Vec;

    quickcheck! {
        fn test_new(data: Vec<u8>) -> bool {
            let native_bs = Bits::new(&data);
            unsafe {
                let mut ffi_bs = mem::zeroed();
                ffi::bs_init(&mut ffi_bs, data.as_ptr(), data.len() as _);
                native_bs.bs_copy() == ffi_bs
            }
        }
    }

    quickcheck! {
        fn test_get_bits(data: Vec<u8>, position: usize, amount: u32) -> bool {
            let amount = amount % 32; // asking for more than 32
            // will cause undefined behavior in the c version
            let mut native_bs = Bits::new_with_pos(&data, position);
            let mut ffi_bs = unsafe { native_bs.bs_copy() };
            native_bs.get_bits(amount) == unsafe {
                ffi::get_bits(&mut ffi_bs, amount as _)
            } && native_bs.position as i32 == ffi_bs.pos
        }
    }
}
