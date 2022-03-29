use hex;
use sha2::Digest;
pub fn hash<D: Digest>(data: &[u8]) -> String {
    let mut hasher = D::new();
    hasher.update(data);
    hex::encode(hasher.finalize().as_slice())
}
