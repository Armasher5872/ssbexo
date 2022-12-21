mod aerials;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
  aerials::install();
  smashes::install();
  specials::install();
  throws::install();
  tilts::install();
}