use super::*;

//ASDI Check
pub unsafe extern "C" fn asdi_check(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[IS_STOP].get_bool() {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    };
}

//ASDI Function
pub unsafe extern "C" fn asdi_function(fighter: &mut L2CFighterCommon) {
    let mut pos = Vector3f {x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor), z: PostureModule::pos_z(fighter.module_accessor)}; // get current pos
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START) && !StopModule::is_damage(fighter.module_accessor) {
        let asdi_stick_x = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_x(fighter.module_accessor)} else {stick_x}; // get stick x length, uses cstick's value if cstick is on (for Double Stick ASDI)
        let asdi_stick_y = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_y(fighter.module_accessor)} else {stick_y}; // get stick y length, uses cstick's value if cstick is on (for Double Stick ASDI)
        let asdi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_auto_mul")); // get base asdi distance
        let asdi_x = asdi * asdi_stick_x*lr; // mul asdi stick_x by total asdi
        let asdi_y = asdi * asdi_stick_y; // mul asdi stick_y by total asdi
        pos.x += asdi_x; //add asdi_x to pos_x
        pos.y += asdi_y; //add asdi_y to pos_y
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    };
}

//Checks if the user got damaged
pub unsafe fn is_damaged(boma: *mut BattleObjectModuleAccessor) -> bool {
    let status = StatusModule::status_kind(boma);
    if FighterStopModuleImpl::is_damage_stop(boma)
    || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
    || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
    || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
    || (*FIGHTER_STATUS_KIND_CAPTURE_PULLED..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status)
    || (*FIGHTER_STATUS_KIND_DOWN..=*FIGHTER_STATUS_KIND_LAY_DOWN).contains(&status)
    || (*FIGHTER_STATUS_KIND_DOWN_DAMAGE..=*FIGHTER_STATUS_KIND_BIND).contains(&status)
    || (*FIGHTER_STATUS_KIND_SLIP..=*FIGHTER_STATUS_KIND_SLIP_WAIT).contains(&status)
    || (*FIGHTER_STATUS_KIND_TREAD_DAMAGE..=*FIGHTER_STATUS_KIND_ICE_JUMP).contains(&status)
    || (*FIGHTER_STATUS_KIND_LINK_FINAL..=*FIGHTER_STATUS_KIND_PIT_FALL).contains(&status)
    || (*FIGHTER_STATUS_KIND_SWALLOWED..=*FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI).contains(&status)
    || (*FIGHTER_STATUS_KIND_CATCHED_REFLET..=*FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND).contains(&status)
    || status == *FIGHTER_STATUS_KIND_GIMMICK_EATEN
    || (*FIGHTER_STATUS_KIND_CAPTURE_ITEM..=*FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP).contains(&status)
    || (*FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER..=*FIGHTER_STATUS_KIND_RIDLEY_FINAL_TARGET_END).contains(&status)
    || (*FIGHTER_STATUS_KIND_CATCHED_RIDLEY..=*FIGHTER_STATUS_KIND_STABBED_DAMAGE).contains(&status)
    || (*FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED..=*FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE).contains(&status)
    || (*FIGHTER_STATUS_KIND_SHEIK_FINAL_CAPTURE..=*FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS).contains(&status)
    || (*FIGHTER_STATUS_KIND_SIMON_FINAL_TARGET_START..=*FIGHTER_STATUS_KIND_YOSHI_FINAL_TARGET_END).contains(&status)
    || (*FIGHTER_STATUS_KIND_SUICIDE_BOMB..=*FIGHTER_STATUS_KIND_TANTAN_FINAL_TARGET_END).contains(&status)
    || (*FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD..=*FIGHTER_STATUS_KIND_EDGE_FINAL_TARGET_END).contains(&status)
    || (*FIGHTER_STATUS_KIND_CAPTURE_TRAIL_KEYHOLE..=*FIGHTER_STATUS_KIND_TRAIL_FINAL_TARGET_END).contains(&status) {
        return true;
    }
    false
}