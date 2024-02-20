use super::*;

unsafe extern "C" fn luigi_special_s_wall_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_exit_status)
    .install()
    ;
}