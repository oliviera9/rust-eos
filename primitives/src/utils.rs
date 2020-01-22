use crate::{
    Error,
    Checksum256,
};

pub fn convert_hex_string_to_checksum256(
    hex: &str,
) -> crate::Result<Checksum256> {
    const NUM_BYTES_IN_CHECKSUM_256: usize = 32;
    match hex::decode(hex) {
        Err(_) => Err(Error::CustomError("✘ Invalid hex!".to_string())),
        Ok(bytes) => {
            match &bytes.len() {
                &NUM_BYTES_IN_CHECKSUM_256 => {
                    let mut array = [0; NUM_BYTES_IN_CHECKSUM_256];
                    let bytes = &bytes[..array.len()];
                    array.copy_from_slice(bytes);
                    Ok(Checksum256::from(array))
                }
                _ => Err(Error::CustomError(
                    "✘ Not enough bytes for checksum 256!".to_string())
                )
            }
        }
    }
}
