use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;

/// function samay समय : () -> float
/// this function returns the current time in milliseconds
pub fn samay(_aadhaar: &mut Aadhaar, _from: usize) -> Result<Mulya, Dosa> {
    let dur: Duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");

    let ms: f64 = dur.as_millis() as f64;
    Ok(Mulya::Ank(ms))
}
