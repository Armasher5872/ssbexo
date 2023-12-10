use super::*;

unsafe extern "C" fn marth_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let motion_kind = MotionModule::motion_kind(module_accessor);
    let frame = MotionModule::frame(module_accessor);
    if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind)
    && frame > 28.0 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
    }
}

pub fn install() {
    Agent::new("marth")
    .on_line(Main, marth_frame)
    .install()
    ;
}