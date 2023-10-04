use super::*;

#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let end_frame = MotionModule::end_frame(boma);
        let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
        if motion_kind == hash40("damage_n_2") {
            if StatusModule::is_situation_changed(boma) {
                if situation_kind != SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
                else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
            if end_frame - frame <= 2.0 {
                if situation_kind != *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                }
                else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        };
        if CHARGE_SHOT_TIMER[entry_id] > 0 {
            CHARGE_SHOT_TIMER[entry_id] -= 1;
        }
        if CHARGE_SHOT_TIMER[entry_id] <= 0
        && HAS_FIRE_CHARGE_SHOT[entry_id] == true {
            HAS_FIRE_CHARGE_SHOT[entry_id] = false;
            fighter.gimmick_flash();
        }
    }
}

pub fn install() {
    install_agent_frames!(
        samusd_frame
    );
}