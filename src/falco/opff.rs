use super::*;

#[fighter_frame( agent = FIGHTER_KIND_FALCO )]
fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
        && StatusModule::is_situation_changed(module_accessor) {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
		};
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if (5.0..=15.0).contains(&frame) {
                REFLECTOR_KNOCKBACK[entry_id] -= 3;
                REFLECTOR_ANGLE[entry_id] -= 1;
            }
            if (16.0..=19.0).contains(&frame) {
                REFLECTOR_KNOCKBACK[entry_id] = 90;
                REFLECTOR_ANGLE[entry_id] = 60;
            }
            if (20.0..=31.0).contains(&frame) {
                REFLECTOR_KNOCKBACK[entry_id] -= 3;
                REFLECTOR_ANGLE[entry_id] -= 1;
            }
        }
        if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
            REFLECTOR_KNOCKBACK[entry_id] = 100;
            REFLECTOR_ANGLE[entry_id] = 60;
        }
    }
}

pub fn install() {
    install_agent_frames!(
        falco_frame
    );
}