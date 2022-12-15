mod aerials;
mod smashes;
mod tilts;

pub fn install() {
  aerials::install();
  smashes::install();
  tilts::install();
}