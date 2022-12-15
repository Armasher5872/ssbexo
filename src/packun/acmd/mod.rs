mod aerials;
mod grounded;
mod smashes;
mod tilts;

pub fn install() {
  aerials::install();
  grounded::install();
  smashes::install();
  tilts::install();
}