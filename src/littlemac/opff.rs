use super::*;

#[fighter_frame( agent = FIGHTER_KIND_LITTLEMAC )]
fn littlemac_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = fighter.global_table[STATUS_KIND].get_i32();
        let frame = MotionModule::frame(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let parried = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        let parry_timer = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        //Hitstun Check
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
            MAC_HITSTUN[get_player_number(boma)] += 1;
        }
        else {
            MAC_HITSTUN[get_player_number(boma)] = 0;
        }
        //Parry Addition
        if [hash40("just_shield_off"), hash40("just_shield")].contains(&motion_kind)
        && (0.0..5.0).contains(&frame)
        && parried != 1 {
            WorkModule::add_float(boma, 34.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
            WorkModule::set_int(boma, 1, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
            WorkModule::set_int(boma, 1, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        }
        if parry_timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        }
        if parry_timer <= 0
        && parried == 1 {
            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        littlemac_frame
    );
}