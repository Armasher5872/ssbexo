use super::*;

//ASDI Check
pub unsafe extern "C" fn asdi_check(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[IS_STOP].get_bool() {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    };
}

//ASDI Function
pub unsafe extern "C" fn asdi_function(fighter: &mut L2CFighterCommon) {
    let mut pos = Vector3f {x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor), z: PostureModule::pos_z(fighter.module_accessor)}; // get current pos
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START) && !StopModule::is_damage(fighter.module_accessor) {
        let asdi_stick_x = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_x(fighter.module_accessor)} else {stick_x}; // get stick x length, uses cstick's value if cstick is on (for Double Stick ASDI)
        let asdi_stick_y = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_y(fighter.module_accessor)} else {stick_y}; // get stick y length, uses cstick's value if cstick is on (for Double Stick ASDI)
        let asdi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_auto_mul")); // get base asdi distance
        let asdi_x = asdi * asdi_stick_x*lr; // mul asdi stick_x by total asdi
        let asdi_y = asdi * asdi_stick_y; // mul asdi stick_y by total asdi
        pos.x += asdi_x; //add asdi_x to pos_x
        pos.y += asdi_y; //add asdi_y to pos_y
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    };
}