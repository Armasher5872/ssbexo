mod tilts;
mod smashes;
mod aerials;
mod throws;
mod specials;

pub fn install() {
  tilts::install();
  smashes::install();
  aerials::install();
  throws::install();
  specials::install();
}