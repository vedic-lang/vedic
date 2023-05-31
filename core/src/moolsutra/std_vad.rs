use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::logger;
use super::super::mulya::Mulya;

/// function vad वद : (string) -> void
/// this function prints the given string
pub fn vad(aadhaar: &mut Aadhaar, from: usize) -> Result<Mulya, Dosa> {
    let args = &aadhaar.rashi[from..aadhaar.rashi_len()];
    let mut output: Vec<String> = vec![];

    for &arg in args.iter() {
        output.push(arg.to_string());
    }

    logger::log(&output.join(" "));
    Ok(Mulya::Na)
}
