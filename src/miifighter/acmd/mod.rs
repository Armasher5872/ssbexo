mod aerials;
mod specials;
mod throws;

pub fn install() {
  aerials::install();
  specials::install();
  throws::install();
}