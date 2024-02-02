use super::*;

pub unsafe extern "C" fn miiswordsman_special_hi1_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_LOOP)(fighter)
}

pub unsafe extern "C" fn miiswordsman_special_hi1_loop_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi1_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(Main, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_LOOP)(fighter)
}

pub unsafe extern "C" fn miiswordsman_special_hi1_loop_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi1_loop_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(End, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_LOOP)(fighter)
}

pub unsafe extern "C" fn miiswordsman_special_hi1_loop_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_END {
        EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP as u32, false, true);
    }
    0.into()
}