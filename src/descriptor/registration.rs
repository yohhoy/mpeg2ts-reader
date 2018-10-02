use std::fmt;
use hex_slice::AsHex;
use super::DescriptorError;

pub struct RegistrationDescriptor<'buf> {
    pub buf: &'buf[u8],
}
impl<'buf> RegistrationDescriptor<'buf> {
    pub const TAG: u8 = 5;
    pub fn new(_tag: u8, buf: &'buf[u8]) -> Result<RegistrationDescriptor<'buf>, DescriptorError> {
        if buf.len() < 4 {
            Err(DescriptorError::NotEnoughData { tag: Self::TAG, actual: buf.len(), expected: 4 })
        } else {
            Ok(RegistrationDescriptor { buf })
        }
    }

    pub fn format_identifier(&self) -> u32 {
        u32::from(self.buf[0]) << 24
            | u32::from(self.buf[1]) << 16
            | u32::from(self.buf[2]) << 8
            | u32::from(self.buf[3])
    }

    pub fn additional_identification_info(&self) -> &[u8] {
        &self.buf[4..]
    }
}
impl<'buf> fmt::Debug for RegistrationDescriptor<'buf> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        f.debug_struct("RegistrationDescriptor")
            .field("format_identifier", &self.format_identifier())
            .field("additional_identification_info", &format!("{:x}", self.additional_identification_info().as_hex()))
            .finish()
    }
}

#[cfg(test)]
mod test {
    use data_encoding::hex;
    use super::*;
    use super::super::{Descriptor, CoreDescriptors};

    #[test]
    fn descriptor() {
        let data = hex::decode(b"050443554549").unwrap();
        let desc = CoreDescriptors::from_bytes(&data).unwrap();
        assert_matches!(desc, CoreDescriptors::Registration(RegistrationDescriptor{ buf: b"CUEI" } ));
    }
}