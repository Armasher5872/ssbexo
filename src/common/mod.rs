pub mod status;
pub mod tech;

pub fn install() {
    status::install();
    tech::install();
    unsafe {
        // Removes phantoms
        skyline::patching::Patch::in_text(0x3e6ce8).data(0x14000012u32);
        // Resets projectile lifetime on parry, rather than using remaining lifetime
        skyline::patching::Patch::in_text(0x33bd358).nop();
        skyline::patching::Patch::in_text(0x33bd35c).data(0x2a0a03e1);
    }
}