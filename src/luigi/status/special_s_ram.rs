use super::*;

unsafe extern "C" fn luigi_special_s_ram_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_exit_status)
    .install()
    ;
}