use super::*;

//Mario Reset Initialization
unsafe extern "C" fn mario_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    mario_var(&mut *boma);
}

//Mario Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_MARIO, 7, false, false))]
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
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_MARIO, 4, false, true)).data(mario_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_MARIO, 36, false, true)).data(mario_on_attack as *const () as u64);
	skyline::install_hook!(mario_death_initialization);
}