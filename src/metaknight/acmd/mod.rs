mod grounded;
mod tilts;
mod smashes;
mod aerials;
mod throws;

pub fn install() {
  grounded::install();
  tilts::install();
  smashes::install();
  aerials::install();
  throws::install();
}