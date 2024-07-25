use super::*;

unsafe extern "C" fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH].contains(&status_kind)
    && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    }
}

unsafe extern "C" fn bayonetta_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret = false;
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if !ret && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6 != 0 {
            fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH.into(), false.into());
            ret = true;
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SMASH) {
            if stick_x > 0.7 {
                fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH.into(), false.into());
                ret = true;
            }
            if stick_y > 0.7 {
                fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH.into(), false.into());
                ret = true;
            }
            if stick_y < -0.7 {
                fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH.into(), false.into());
                ret = true;
            }
        }
    }
    ret.into()
}

unsafe extern "C" fn bayonetta_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(bayonetta_check_special_command as *const () as _));
}

pub fn install() {
    Agent::new("bayonetta")
    .on_start(bayonetta_init)
    .on_line(Main, bayonetta_frame)
    .install()
    ;
}