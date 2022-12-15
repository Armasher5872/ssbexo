mod grounded;
mod tilts;
mod aerials;
mod throws;
mod specials;

pub fn install() {
  grounded::install();
  tilts::install();
  aerials::install();
  throws::install();
  specials::install();
}