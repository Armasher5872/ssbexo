use super::*;

unsafe extern "C" fn pikachu_special_s_hold_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_hold_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("pikachu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_pre_status)
    .status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_init_status)
    .status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_main_status)
    .status(Exec, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_exec_status)
    .status(End, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_end_status)
    .status(Exit, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pikachu_special_s_hold_exit_status)
    .install()
    ;
}