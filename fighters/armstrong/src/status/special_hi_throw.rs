use super::*;

unsafe extern "C" fn armstrong_special_hi_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.03);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.02);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y*0.4);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x*0.25);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.0);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_SET_IGNORE_CATCHING, true);
    sv_module_access::capture(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_frames = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    let damage_multiplier = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if charge_frames > 0 {
        AttackModule::set_power_up(fighter.module_accessor, damage_multiplier);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        armstrong_special_hi_throw_sub_status(fighter, false.into());
    }
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_THROWN, false);
    }
    grabbed_anim_selector(fighter, "thrown_f", 0.0, 1.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_throw"), 0.0, 1.0, false, 0.0, false, false);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(armstrong_special_hi_throw_sub_status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_hi_throw_main_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_hi_throw_sub_status(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    if !bool_check.get_bool() {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_CHANGE_KINE) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_FALL) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_CHANGE_KINE);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GANON_SPECIAL_HI_THROW_FALL);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_THROW) {
        if capture_id != 0x50000000 {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            VisibilityModule::set_whole(capture_boma, true);
            AttackModule::hit_absolute_joint(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, capture_id as u32, Hash40::new("throw"), 0, 0);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_THROW);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}


unsafe extern "C" fn armstrong_special_hi_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    armstrong_clear_charge(fighter.module_accessor);
    if CatchModule::is_catch(fighter.module_accessor) {
        let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
        if capture_id != 0x50000000 {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            let pos = *PostureModule::pos(fighter.module_accessor);
            PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
        }
        CatchModule::set_send_cut_event(fighter.module_accessor, true);
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_THROW);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    armstrong_clear_charge(fighter.module_accessor);
    if CatchModule::is_catch(fighter.module_accessor) {
        let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
        if capture_id != 0x50000000 {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            let pos = *PostureModule::pos(fighter.module_accessor);
            PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
        }
        CatchModule::set_send_cut_event(fighter.module_accessor, true);
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_THROW);
    0.into()
}

pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    Agent::new("ganon")
    .set_costume(costume.to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_exit_status)
    .install()
    ;
}