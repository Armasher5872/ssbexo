use super::*;

#[fighter_frame( agent = FIGHTER_KIND_YOSHI )]
fn yoshi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        if motion_kind == hash40("special_air_n_tether")
        && frame >= 29.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_tether_wait"), 0.0, 1.0, false, 0.0, false, false);
        }
        if motion_kind == hash40("special_air_n_tether_wait") {
            if frame >= 78.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_tether_wait"), 1.0, 1.0, false, 0.0, false, false);
            }
            if fighter.jump_cancel() {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        yoshi_frame
    );
}