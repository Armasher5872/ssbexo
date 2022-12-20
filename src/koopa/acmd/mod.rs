mod aerials;
mod grounded;
mod smashes;
mod specials;
mod tilts;

pub fn install() {
  aerials::install();
  grounded::install();
  smashes::install();
  specials::install();
  tilts::install();
}