mod aerials;
mod grounded;
mod smashes;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    grounded::install();
    smashes::install();
    throws::install();
    tilts::install();
}