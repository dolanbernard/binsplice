
use crate::error::IpsPatchError;

#[derive(Copy, Clone, Debug)]
pub struct IpsPatcher<'a> {
    patch: &'a Vec<u8>,
    cursor: usize,
}

impl<'a> IpsPatcher<'a> {
    pub fn patch(&mut self, patch_me: &mut Vec<u8>) -> Result<usize, IpsPatchError> {
        let mut bytes_written = 0;
        assert_eq!(PATCH_HEADER, self.advance(5));
        while self.cursor < self.patch.len() {
            let write_pos = read_usize(self.advance(3));
            if write_pos == read_usize(EOF) {
                break;
            }
            let write_len = read_usize(self.advance(2));
            if write_len == 0 {
                let rle_len = read_usize(self.advance(2));
                let rle_payload = self.advance(1)[0];
                bytes_written += write_rle(write_pos, patch_me, rle_payload, rle_len);
            } else {
                let payload = self.advance(write_len);
                bytes_written += write_payload(write_pos, patch_me, payload);
            }
        }
        Ok(bytes_written)
    }

    pub fn new(data: &'a mut Vec<u8>, start: usize) -> Self {
        IpsPatcher {
            patch: data,
            cursor: start,
        }
    }

    fn advance(&mut self, len: usize) -> &'a [u8] {
        let slice = &self.patch[self.cursor..(self.cursor + len)];
        self.cursor += len;
        slice
    }
}

fn read_usize(be_bytes: &[u8]) -> usize {
    //usize::from_be_bytes(be_bytes.try_into().unwrap())
    let mut result = 0;
    for i in (0..be_bytes.len()).rev() {
        result += be_bytes[i] as usize;
    }
    result
}

fn write_payload(offset: usize, buffer: &mut Vec<u8>, payload: &[u8]) -> usize {
    let max_index = offset + payload.len();
    if max_index > buffer.len() {
        buffer.resize(buffer.len() + (max_index - buffer.len()), 0);
    }
    buffer[offset..max_index].copy_from_slice(payload);
    payload.len()
}

fn write_rle(offset: usize, buffer: &mut Vec<u8>, rle_payload: u8, rle_len: usize) -> usize {
    let max_index = offset + rle_len;
    if max_index > buffer.len() {
        buffer.resize(buffer.len() + (max_index - buffer.len()), 0);
    }
    buffer.iter_mut().skip(offset).take(rle_len).for_each(|b| *b = rle_payload);
    rle_len
}

const PATCH_HEADER: &'static [u8; 5] = &[0x50, 0x41, 0x54, 0x43, 0x48];
const EOF: &'static [u8; 3] = &[0x45, 0x4F, 0x46];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patching() {
        let mut data: Vec<u8> = (0..100).into_iter().collect();
        let mut patched_data = data.clone();
        for i in 5..14 {
            patched_data[i] = 13 - (i - 5) as u8;
        }

        let mut patch = Vec::new();
        patch.extend_from_slice(PATCH_HEADER);
        patch.extend_from_slice(&[0, 0, 5]);
        patch.extend_from_slice(&[0, 9]);
        let payload: Vec<u8> = [5, 6, 7, 8, 9, 10, 11, 12, 13].into_iter().rev().collect();
        patch.extend_from_slice(&payload);
        patch.extend_from_slice(EOF);

        let mut uut = IpsPatcher::new(&mut patch, 0);
        assert_eq!(9, uut.patch(&mut data).unwrap());
        assert_eq!(patched_data.len(), data.len());
        assert_eq!(patched_data, data);
    }

    #[test]
    fn test_rle_patching() {
        let mut data: Vec<u8> = (0..100).into_iter().collect();
        let mut patched_data = data.clone();
        for i in 42..69 {
            patched_data[i] = 69;
        }

        let mut patch = Vec::new();
        patch.extend_from_slice(PATCH_HEADER);
        patch.extend_from_slice(&[0, 0, 42]);
        patch.extend_from_slice(&[0, 0]);
        patch.extend_from_slice(&[0, 69-42, 69]);
        patch.extend_from_slice(EOF);

        let mut uut = IpsPatcher::new(&mut patch, 0);
        assert_eq!(69-42, uut.patch(&mut data).unwrap());
        assert_eq!(patched_data.len(), data.len());
        assert_eq!(patched_data, data);
    }

    #[test]
    fn test_patch_extending() {
        let mut data: Vec<u8> = (0..100).into_iter().collect();
        let mut patched_data = data.clone();
        patched_data.extend_from_slice(&[100, 101, 102, 103]);

        let mut patch = Vec::new();
        patch.extend_from_slice(PATCH_HEADER);
        patch.extend_from_slice(&[0, 0, 100]);
        patch.extend_from_slice(&[0, 4]);
        patch.extend_from_slice(&[100, 101, 102, 103]);
        patch.extend_from_slice(EOF);

        let mut uut = IpsPatcher::new(&mut patch, 0);
        assert_eq!(4, uut.patch(&mut data).unwrap());
        assert_eq!(patched_data.len(), data.len());
        assert_eq!(patched_data, data);
        println!("{:?}", data);
    }

    #[test]
    fn test_rle_patch_extending() {
        let mut data: Vec<u8> = (0..100).into_iter().collect();
        let mut patched_data = data.clone();
        patched_data.extend_from_slice(&[69, 69, 69, 69]);

        let mut patch = Vec::new();
        patch.extend_from_slice(PATCH_HEADER);
        patch.extend_from_slice(&[0, 0, 100]);
        patch.extend_from_slice(&[0, 0]);
        patch.extend_from_slice(&[0, 4, 69]);
        patch.extend_from_slice(EOF);

        let mut uut = IpsPatcher::new(&mut patch, 0);
        assert_eq!(4, uut.patch(&mut data).unwrap());
        assert_eq!(patched_data.len(), data.len());
        assert_eq!(patched_data, data);
        println!("{:?}", data);
    }
}
