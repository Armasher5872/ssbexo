mod grounded;
mod tilts;
mod smashes;
mod aerials;
mod throws;
mod specials;

pub fn install() {
  grounded::install();
  tilts::install();
  smashes::install();
  aerials::install();
  throws::install();
  specials::install();
}