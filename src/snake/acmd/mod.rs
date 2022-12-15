mod aerials;
mod smashes;
mod specials;
mod tilts;

pub fn install() {
  aerials::install();
  smashes::install();
  specials::install();
  tilts::install();
}