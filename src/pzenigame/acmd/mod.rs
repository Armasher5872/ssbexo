mod aerials;
mod smashes;
mod specials;
mod throws;

pub fn install() {
    aerials::install();
    smashes::install();
    specials::install();
    throws::install();
}