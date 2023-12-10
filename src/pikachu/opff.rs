use super::*;

unsafe extern "C" fn pikachu_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
        if frame < 1.0 {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        }
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD)
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
            WorkModule::inc_int(boma, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
        }
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT) > 1 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
        }
    };
    if status_kind != *FIGHTER_STATUS_KIND_ATTACK {
        WorkModule::set_int(boma, 0, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
    }
}

pub fn install() {
    Agent::new("pikachu")
    .on_line(Main, pikachu_frame)
    .install()
    ;
}