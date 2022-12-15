mod grounded;
mod tilts;
mod smashes;
mod aerials;

pub fn install() {
  grounded::install();
  tilts::install();
  smashes::install();
  aerials::install();
}