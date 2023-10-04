pub mod status;
pub mod tech;

pub fn install() {
    status::install();
    tech::install();
}