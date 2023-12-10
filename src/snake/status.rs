use super::*;

unsafe extern "C" fn snake_forward_smash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter)
}

unsafe extern "C" fn snake_forward_smash_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
    SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
    SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 0;
    original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter)
}

pub fn install() {
    Agent::new("snake")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, snake_forward_smash_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, snake_forward_smash_end_status)
    .install()
    ;
}