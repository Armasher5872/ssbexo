use super::*;

unsafe extern "C" fn metaknight_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kinetic;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S);
        kinetic = *FIGHTER_KINETIC_TYPE_FALL;
    }
    else {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S);
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
    }
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), kinetic, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn metaknight_special_s_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn metaknight_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_start"), L2CValue::Hash40s("special_air_s_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_s_loop as *const () as _))
}

unsafe extern "C" fn metaknight_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn metaknight_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn metaknight_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn metaknight_special_s_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn metaknight_galaxia_beam_shoot_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn metaknight_galaxia_beam_shoot_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    PostureModule::set_lr(weapon.module_accessor, PostureModule::lr(weapon.module_accessor));
    ModelModule::set_scale(weapon.module_accessor, 0.001);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn metaknight_galaxia_beam_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("speed_max"));
    let lr = PostureModule::lr(weapon.module_accessor);
    if WorkModule::is_flag(owner_boma, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S)
    || weapon.is_situation(*SITUATION_KIND_AIR) {
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, -speed_max);
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, -speed_max);
    }
    if !WorkModule::is_flag(owner_boma, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S)
    || weapon.is_situation(*SITUATION_KIND_GROUND) {
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
    }
    KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_NORMAL);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    if StopModule::is_stop(weapon.module_accessor) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        galaxia_beam_removal(weapon);
    }
    weapon.fastshift(L2CValue::Ptr(metaknight_galaxia_beam_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_galaxia_beam_shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT)
    || GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK)
    || WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        galaxia_beam_removal(weapon);
    }
    0.into()
}

unsafe extern "C" fn metaknight_galaxia_beam_shoot_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn metaknight_galaxia_beam_shoot_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    0.into()
}

/*
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall
L2CWeaponElightExprosiveshot::status::Fly_main(L2CWeaponElightExprosiveshot *this,L2CValue *return_value) {
    lib::L2CValue::L2CValue(&LStack_e0,0x13c69415a9);
    lib::L2CValue::L2CValue(&LStack_80,0x419cd3efe);
    uVar4 = lib::L2CValue::as_integer(&LStack_e0);
    uVar5 = lib::L2CValue::as_integer(&LStack_80);
    iVar2 = app::lua_bind::WorkModule__get_param_int_impl(this->moduleAccessor,uVar4,uVar5);
    lib::L2CValue::L2CValue(&LStack_70,iVar2);
    lib::L2CValue::~L2CValue(&LStack_80);
    lib::L2CValue::~L2CValue(&LStack_e0);
    lib::L2CValue::L2CValue(&LStack_e0,_WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    iVar2 = lib::L2CValue::as_integer(&LStack_70);
    iVar3 = lib::L2CValue::as_integer(&LStack_e0);
    app::lua_bind::WorkModule__set_int_impl(this->moduleAccessor,iVar2,iVar3);
    lib::L2CValue::~L2CValue(&LStack_e0);
    lib::L2CValue::L2CValue(&LStack_e0,_WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    iVar2 = lib::L2CValue::as_integer(&LStack_70);
    iVar3 = lib::L2CValue::as_integer(&LStack_e0);
    app::lua_bind::WorkModule__set_int_impl(this->moduleAccessor,iVar2,iVar3);
    lib::L2CValue::~L2CValue(&LStack_e0);
    lib::L2CValue::L2CValue(&LStack_e0,0x3538a83b3);
    lib::L2CValue::L2CValue(&LStack_80,0.0);
    lib::L2CValue::L2CValue(&LStack_90,1.0);
    lib::L2CValue::L2CValue(&LStack_a0,false);
    HVar6 = lib::L2CValue::as_hash(&LStack_e0);
    fVar10 = (float)lib::L2CValue::as_number(&LStack_80);
    fVar11 = (float)lib::L2CValue::as_number(&LStack_90);
    bVar1 = lib::L2CValue::as_bool(&LStack_a0);
    app::lua_bind::MotionModule__change_motion_impl
              (this->moduleAccessor,HVar6,fVar10,fVar11,(bool)(bVar1 & 1),0.0,false,false);
    lib::L2CValue::~L2CValue(&LStack_a0);
    lib::L2CValue::~L2CValue(&LStack_90);
    lib::L2CValue::~L2CValue(&LStack_80);
    lib::L2CValue::~L2CValue(&LStack_e0);
    lib::L2CValue::L2CValue(&LStack_e0,0x13c69415a9);
    lib::L2CValue::L2CValue(&LStack_90,0x50f26fef6);
    uVar4 = lib::L2CValue::as_integer(&LStack_e0);
    uVar5 = lib::L2CValue::as_integer(&LStack_90);
    fVar10 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar4,uVar5);
    lib::L2CValue::L2CValue(&LStack_80,fVar10);
    lib::L2CValue::~L2CValue(&LStack_90);
    lib::L2CValue::~L2CValue(&LStack_e0);
    lib::L2CValue::L2CValue(&LStack_e0,0x13c69415a9);
    lib::L2CValue::L2CValue(&LStack_a0,0x500814509);
    uVar4 = lib::L2CValue::as_integer(&LStack_e0);
    uVar5 = lib::L2CValue::as_integer(&LStack_a0);
    fVar10 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar4,uVar5);
    lib::L2CValue::L2CValue(&LStack_90,fVar10);
    lib::L2CValue::~L2CValue(&LStack_a0);
    lib::L2CValue::~L2CValue(&LStack_e0);
    lib::L2CValue::L2CValue(&LStack_b0,(L2CValue *)&LStack_80);
    lib::L2CValue::L2CValue(&LStack_c0,0);
    lua2cpp::L2CFighterBase::Vector2__create(this,SUB81(&LStack_b0,0),SUB81(&LStack_c0,0));
    lib::L2CValue::~L2CValue(&LStack_c0);
    lib::L2CValue::~L2CValue(&LStack_b0);
    pLVar7 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x18cdc1683);
    this_00 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x1fbdb2615);
    this_01 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x18cdc1683);
    auVar14 = lib::L2CValue::operator[](&LStack_a0,0x1fbdb2615);
    lib::L2CAgent::math_rad((L2CAgent *)&LStack_90,SUB168(auVar14 >> 0x40,0));
    fVar10 = (float)lib::L2CValue::as_number(this_01);
    fVar11 = (float)lib::L2CValue::as_number(SUB168(auVar14,0));
    fVar12 = (float)lib::L2CValue::as_number(&LStack_f0);
    uVar13 = app::sv_math::vec2_rot(fVar10,fVar11,fVar12);
    lib::L2CValue::L2CValue(&LStack_e0,(float)uVar13);
    lib::L2CValue::L2CValue(&LStack_d0,(float)((ulong)uVar13 >> 0x20));
    lib::L2CValue::operator=(pLVar7,(L2CValue *)&LStack_e0);
    lib::L2CValue::operator=(this_00,(L2CValue *)&LStack_d0);
    lib::L2CValue::~L2CValue(&LStack_d0);
    lib::L2CValue::~L2CValue(&LStack_e0);
    lib::L2CValue::~L2CValue(&LStack_f0);
    pLVar7 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x18cdc1683);
    fVar10 = (float)app::lua_bind::PostureModule__lr_impl(this->moduleAccessor);
    lib::L2CValue::L2CValue(&LStack_f0,fVar10);
    lib::L2CValue::operator*(pLVar7,(L2CValue *)&LStack_f0);
    pLVar7 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x18cdc1683);
    lib::L2CValue::operator=(pLVar7,(L2CValue *)&LStack_e0);
    lib::L2CValue::~L2CValue(&LStack_e0);
    lib::L2CValue::~L2CValue(&LStack_f0);
    lib::L2CValue::L2CValue(&LStack_e0,_WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    pLVar8 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x18cdc1683);
    pLVar9 = (L2CValue *)lib::L2CValue::operator[](&LStack_a0,0x1fbdb2615);
    lib::L2CAgent::clear_lua_stack((L2CAgent *)this);
    lib::L2CAgent::push_lua_stack((L2CAgent *)this,(L2CValue *)&LStack_e0);
    lib::L2CAgent::push_lua_stack((L2CAgent *)this,pLVar8);
    lib::L2CAgent::push_lua_stack((L2CAgent *)this,pLVar9);
    app::sv_kinetic_energy::set_speed(this->luaStateAgent);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, vec.x)
    weapon.fastshift(L2CValue::Ptr(metaknight_galaxia_beam_air_fly_main_loop as *const () as _))
}
*/

unsafe extern "C" fn metaknight_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kinetic;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
    }
    else {
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
    }
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), kinetic, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

//Special Hi Main, resets your jump count
unsafe extern "C" fn metaknight_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    ret
}

//Special Hi Loop, resets your jump count
unsafe extern "C" fn metaknight_special_hi_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(Main, fighter, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP)(fighter);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    ret
}
//Glide Start Main
unsafe extern "C" fn metaknight_glide_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    fighter.status_GlideStart()
}

//Glide End Main
unsafe extern "C" fn metaknight_glide_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_end"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_glide_end_main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_glide_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

//Glide Attack Main
unsafe extern "C" fn metaknight_glide_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_attack"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_glide_attack_main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_glide_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, metaknight_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, metaknight_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, metaknight_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, metaknight_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, metaknight_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, metaknight_special_s_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, metaknight_special_hi_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, metaknight_special_hi_main_status)
    .status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP, metaknight_special_hi_loop_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_START, metaknight_glide_start_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_END, metaknight_glide_end_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, metaknight_glide_attack_main_status)
    .install()
    ;
    Agent::new("metaknight_galaxiabeam")
    .status(Pre, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, metaknight_galaxia_beam_shoot_pre_status)
    .status(Init, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, metaknight_galaxia_beam_shoot_init_status)
    .status(Main, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, metaknight_galaxia_beam_shoot_main_status)
    .status(Exec, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, metaknight_galaxia_beam_shoot_exec_status)
    .status(End, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, metaknight_galaxia_beam_shoot_end_status)
    .install()
    ;
    Agent::new("metaknight_galaxiabeamair")
    .status(Pre, WEAPON_METAKNIGHT_GALAXIA_BEAM_AIR_STATUS_KIND_FLY, metaknight_galaxia_beam_air_fly_pre_status)
    .status(Init, WEAPON_METAKNIGHT_GALAXIA_BEAM_AIR_STATUS_KIND_FLY, metaknight_galaxia_beam_air_fly_init_status)
    .status(Main, WEAPON_METAKNIGHT_GALAXIA_BEAM_AIR_STATUS_KIND_FLY, metaknight_galaxia_beam_air_fly_main_status)
    .status(Exec, WEAPON_METAKNIGHT_GALAXIA_BEAM_AIR_STATUS_KIND_FLY, metaknight_galaxia_beam_air_fly_exec_status)
    .status(End, WEAPON_METAKNIGHT_GALAXIA_BEAM_AIR_STATUS_KIND_FLY, metaknight_galaxia_beam_air_fly_end_status)
    .install()
    ;
}