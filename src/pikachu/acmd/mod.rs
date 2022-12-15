mod grounded;
mod smashes;
mod aerials;

pub fn install() {
  grounded::install();
  smashes::install();
  aerials::install();
}