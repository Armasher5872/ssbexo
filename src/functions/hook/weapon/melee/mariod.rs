pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x3441a18).nop(); //The following removes a horizontal speed initialization so it can be assigned dynamically in a weapon init status
    let _ = skyline::patching::Patch::in_text(0x3441a3c).nop(); //The following removes the initial gravity acceleration so it can be assigned dynamically in a weapon init status
}