use super::*;

const MARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcb9730; //Mario only

//Mario Reset Initialization
unsafe extern "C" fn mario_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    mario_var(&mut *boma);
}

//Mario Death Initialization
#[skyline::hook(offset = MARIO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mario_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    mario_var(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
    WorkModule::set_int(boma, 0, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    WorkModule::set_int(boma, 0, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_REMOVE);
    EffectModule::remove_common(boma, Hash40::new("charge_max"));
}

//Mario On Attack
unsafe extern "C" fn mario_on_attack(_vtable: u64, fighter: &mut Fighter, _log: u64) {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP].contains(&status_kind) {
        WorkModule::on_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT);
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fe3160).data(mario_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x4fe3260).data(mario_on_attack as *const () as u64);
	skyline::install_hook!(mario_death_initialization);
}