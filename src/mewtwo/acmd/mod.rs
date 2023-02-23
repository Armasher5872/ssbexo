mod aerials;
mod grounded;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
  aerials::install();
  grounded::install();
  smashes::install();
  specials::install();
  throws::install();
  tilts::install();
}