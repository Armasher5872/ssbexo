mod grounded;
mod smashes;
mod throws;
mod specials;

pub fn install() {
  grounded::install();
  smashes::install();
  throws::install();
  specials::install();
}