mod grounded;
mod tilts;
mod smashes;
mod aerials;
mod specials;

pub fn install() {
  grounded::install();
  tilts::install();
  smashes::install();
  aerials::install();
  specials::install();
}