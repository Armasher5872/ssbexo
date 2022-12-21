mod aerials;
mod smashes;
mod throws;

pub fn install() {
  aerials::install();
  smashes::install();
  throws::install();
}