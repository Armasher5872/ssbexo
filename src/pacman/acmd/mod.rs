mod aerials;
mod smashes;
mod throws;
mod tilts;

pub fn install() {
  aerials::install();
  smashes::install();
  throws::install();
  tilts::install();
}