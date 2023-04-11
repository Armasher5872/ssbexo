use {
    crate::functions::variables::*,
    smash::{
        app::lua_bind::*,
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
    },
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_FOX )]
fn fox_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let stick_y = ControlModule::get_stick_y(module_accessor);
        if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)
            && !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) 
            && ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) 
                && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                };
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                };
            }
            if frame >= 4.0
            && ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) 
                && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                };
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                };
            }
            if frame > 19.0 
            && stick_y < -0.2
            && GroundModule::is_passable_ground(module_accessor) {
                WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
                WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
                if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS)
                && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
                }
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        fox_frame
    );
}