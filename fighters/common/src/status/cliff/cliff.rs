//Credited to WuBoyTH
use super::*;

#[skyline::hook(offset = 0x617aa4, inline)]
unsafe extern "C" fn reverse_trump_logic(ctx: &mut skyline::hooks::InlineCtx) {
    let opponent_battle_object = ctx.registers[23].x() as *mut BattleObject;
    WorkModule::on_flag((*opponent_battle_object).module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_ROB);
}

//Installation
pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x617a90).nop(); //Removes WorkModule::on_flag(boma, *FIGHTER_STATUS_CLIFF_FLAG_TO_FALL);
    let _ = skyline::patching::Patch::in_text(0x617aa4).nop(); //Removes WorkModule::on_flag(boma, *FIGHTER_STATUS_CLIFF_FLAG_TO_ROB);
	skyline::install_hook!(reverse_trump_logic);
}