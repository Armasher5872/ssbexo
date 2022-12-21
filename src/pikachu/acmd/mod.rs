mod grounded;
mod smashes;
mod aerials;
mod throws;

pub fn install() {
  grounded::install();
  smashes::install();
  aerials::install();
  throws::install();
}