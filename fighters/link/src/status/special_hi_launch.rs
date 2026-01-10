use super::*;

unsafe extern "C" fn link_special_hi_launch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_launch_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.5);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_launch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let e1;
    let e2;
    let e3;
    let e4;
    if lr < 0.0 {
        e1 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("link_revali_gale_wind"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &Vector3f{x: 0.0, y: -90.0, z: 270.0}, 2.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        e2 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("link_revali_gale_wind"), Hash40::new("top"), &Vector3f{x: 0.0, y: 24.0, z: 0.0}, &Vector3f{x: 0.0, y: -90.0, z: 270.0}, 2.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        e3 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("link_revali_gale_wind"), Hash40::new("top"), &Vector3f{x: 0.0, y: 36.0, z: 0.0}, &Vector3f{x: 0.0, y: -90.0, z: 270.0}, 2.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        e4 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("link_revali_gale_wind"), Hash40::new("top"), &Vector3f{x: 0.0, y: 48.0, z: 0.0}, &Vector3f{x: 0.0, y: -90.0, z: 270.0}, 2.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    }
    else {
        e1 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("link_revali_gale_wind"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &Vector3f{x: 0.0, y: 90.0, z: 270.0}, 2.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        e2 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("link_revali_gale_wind"), Hash40::new("top"), &Vector3f{x: 0.0, y: 24.0, z: 0.0}, &Vector3f{x: 0.0, y: 90.0, z: 270.0}, 2.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        e3 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("link_revali_gale_wind"), Hash40::new("top"), &Vector3f{x: 0.0, y: 36.0, z: 0.0}, &Vector3f{x: 0.0, y: 90.0, z: 270.0}, 2.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        e4 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("link_revali_gale_wind"), Hash40::new("top"), &Vector3f{x: 0.0, y: 48.0, z: 0.0}, &Vector3f{x: 0.0, y: 90.0, z: 270.0}, 2.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    }
    WorkModule::set_int(fighter.module_accessor, e1 as i32, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_1);
    WorkModule::set_int(fighter.module_accessor, e2 as i32, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_2);
    WorkModule::set_int(fighter.module_accessor, e3 as i32, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_3);
    WorkModule::set_int(fighter.module_accessor, e4 as i32, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_4);
    fighter.sub_change_motion_by_situation(Hash40::new("special_hi").into(), Hash40::new("special_air_hi").into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_launch_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_launch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let special_hi_charge_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FRAME);
    let e1 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_1);
    let e2 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_2);
    let e3 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_3);
    let e4 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_4);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_JUMP) {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 1.0+((special_hi_charge_frame/40) as f32), 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.03);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_JUMP);
    }
    if current_frame >= 35.0 {
        EffectModule::set_alpha(fighter.module_accessor, e1 as u32, 1.0-((current_frame-34.0)/15.0));
        EffectModule::set_alpha(fighter.module_accessor, e2 as u32, 1.0-((current_frame-34.0)/15.0));
        EffectModule::set_alpha(fighter.module_accessor, e3 as u32, 1.0-((current_frame-34.0)/15.0));
        EffectModule::set_alpha(fighter.module_accessor, e4 as u32, 1.0-((current_frame-34.0)/15.0));
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_launch_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_special_hi_launch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FRAME);
    EFFECT_OFF_KIND(fighter, Hash40::new("link_revali_gale_wind"), true, true);
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_1) as u32, true, true);
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_2) as u32, true, true);
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_3) as u32, true, true);
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_4) as u32, true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_1);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_2);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_3);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_4);
    0.into()
}

unsafe extern "C" fn link_special_hi_launch_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FRAME);
    EFFECT_OFF_KIND(fighter, Hash40::new("link_revali_gale_wind"), true, true);
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_1) as u32, true, true);
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_2) as u32, true, true);
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_3) as u32, true, true);
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_4) as u32, true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_1);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_2);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_3);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_4);
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH, link_special_hi_launch_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH, link_special_hi_launch_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH, link_special_hi_launch_main_status)
    .status(Exec, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH, link_special_hi_launch_exec_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH, link_special_hi_launch_end_status)
    .status(Exit, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH, link_special_hi_launch_exit_status)
    .install()
    ;
}