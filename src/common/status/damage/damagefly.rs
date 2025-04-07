use super::*;

//static mut FIGHTER_STATUS_DAMAGE_CORRECT_DAMAGE_VECTOR_COMMON_INLINE: usize = 0x16344;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageFly_Main)]
unsafe fn status_damagefly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    asdi_check(fighter);
    asdi_function(fighter);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT) {
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    }
    //Fighter Specific
    if fighter_kind == *FIGHTER_KIND_PICKEL {
        if fighter.global_table[IS_STOP].get_bool() {
            macros::COL_NORMAL(fighter);
            macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.25);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
        if !fighter.status_DamageFinishCamera_exec().get_bool() {
            return 0.into();
        }
        fighter.status_DamageFly_Common();
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ADJUST_VECTOR);
    }
    else {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL) {
            if MotionModule::is_end(fighter.module_accessor) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                    fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
                }
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0 && stick_y < -0.66 && get_sum_speed_y <= -0.5 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
        if fighter.sub_DamageFlyCommon().get_bool() {
            return 0.into();
        }
        if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
            if fighter.sub_AirChkDamageReflectWall().get_bool() || fighter.sub_AirChkDamageReflectCeil().get_bool() || fighter.sub_AirChkDamageReflectFloor().get_bool() {
                return 0.into();
            }
        }
        fighter.FighterStatusDamage__correctDamageVectorEffect(false.into());
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_DamageFly)]
unsafe fn status_end_damagefly(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(fighter.module_accessor, flags[x]);
    }
    //Fighter Specific
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
    if status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY {
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            fighter.sub_end_damage_fly_roll_to_dead();
        }
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
    }
    fighter.check_ryu_final_damage_03(true.into());
    0.into()
}

/*
unsafe extern "C" fn fighter_status_damage_correct_damage_vector_common_inline(fighter: &mut L2CFighterCommon) -> f32 {
    WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_correction_max"))
}
*/

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        /*
        unsafe {
            let common_offset = (*info.module.ModuleObject).module_base as usize;
            FIGHTER_STATUS_DAMAGE_CORRECT_DAMAGE_VECTOR_COMMON_INLINE += common_offset;
            skyline::hooks::A64InlineHook(
                FIGHTER_STATUS_DAMAGE_CORRECT_DAMAGE_VECTOR_COMMON_INLINE as u64 as _,
                fighter_status_damage_correct_damage_vector_common_inline as _
            );
        }
        */
        skyline::install_hooks!(
            status_damagefly_main,
            status_end_damagefly
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}