use super::*;

unsafe extern "C" fn metaknight_glide_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    fighter.status_GlideStart()
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_START, metaknight_glide_start_main_status)
    .install()
    ;
}