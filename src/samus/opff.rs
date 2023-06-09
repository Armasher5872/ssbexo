use super::*;

#[fighter_frame( agent = FIGHTER_KIND_SAMUS )]
fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A].contains(&status_kind)
        && StatusModule::is_situation_changed(boma) {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
            WorkModule::set_float(boma, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
		};
    }
}

pub fn install() {
    install_agent_frames!(
        samus_frame
    );
}