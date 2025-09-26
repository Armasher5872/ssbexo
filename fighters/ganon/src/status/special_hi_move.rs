use super::*;

unsafe extern "C" fn ganon_special_hi_move_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_BRAKE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_ATTACK);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_move_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_charged = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
    let rot_angle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE) as f32;
    let speed: f32 = if is_charged {1.0} else {3.0};
    let speed_x = (rot_angle+90.0).to_radians().sin()*speed;
    let speed_y = (rot_angle-90.0).to_radians().cos()*speed;
    let brake = if is_charged {0.0} else {0.04};
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, brake, brake);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed, speed);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_move_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let pos = PostureModule::pos(fighter.module_accessor);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        WorkModule::set_int(fighter.module_accessor, 30, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    }
    else {
        let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("ganon_entry"), Hash40::new("hip"), &Vector3f{x: (*pos).x, y: (*pos).y, z: 0.0}, &Vector3f::zero(), 0.5, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::set_rate(fighter.module_accessor, effect as u32, 0.6);
        EffectModule::set_rgb(fighter.module_accessor, effect as u32, 1.0, 0.0, 0.325);
        WorkModule::set_int(fighter.module_accessor, effect as i32, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
        WorkModule::set_int(fighter.module_accessor, 110, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    }
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    JostleModule::set_status(fighter.module_accessor, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_move"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_hi_move_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_hi_move_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let special_hi_move_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    if special_hi_move_frame <= 0 {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
        WorkModule::set_float(fighter.module_accessor, 30.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ganon_special_hi_move_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_hi_move_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    if special_hi_move_frame > 0 {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    }
    0.into()
}

unsafe extern "C" fn ganon_special_hi_move_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING].contains(&status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
    }
    EFFECT_OFF_KIND(fighter, Hash40::new("ganon_entry"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_move_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING].contains(&status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
    }
    EFFECT_OFF_KIND(fighter, Hash40::new("ganon_entry"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    GroundModule::set_ignore_boss(fighter.module_accessor, false);
    JostleModule::set_status(fighter.module_accessor, true);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE, ganon_special_hi_move_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE, ganon_special_hi_move_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE, ganon_special_hi_move_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE, ganon_special_hi_move_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE, ganon_special_hi_move_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE, ganon_special_hi_move_exit_status)
    .install()
    ;
}