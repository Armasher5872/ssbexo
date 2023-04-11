pub mod ext;
pub mod hook;
pub mod offsets;
pub mod params;
pub mod reset;
pub mod singletons;
pub mod util;
pub mod variables;

pub fn install() {
	ext::install();
	hook::install();
	params::install();
	reset::install();
}