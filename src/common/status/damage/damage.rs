use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Damage_Main)]
unsafe fn status_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let is_end = MotionModule::is_end(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let break_to_death = fighter.FighterStatusDamage__check_dolly_stadium_wall_break_to_death();
    let damage_fly_reflect_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_reflect_speed"));
    let get_speed_x = {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        smash::app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)
    };
    let mut fall_check;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    asdi_check(fighter);
    asdi_function(fighter);
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    }
    if fighter_kind == *FIGHTER_KIND_PICKEL {
        if fighter.global_table[IS_STOP].get_bool() {
            macros::COL_NORMAL(fighter);
            macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.25);
        }
    }
    if situation_kind != *SITUATION_KIND_AIR {
        if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            fall_check = false;
        }
        else {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                fall_check = false;
            }
            if !is_end {
                fall_check = false;
            }
            else {
                fall_check = (situation_kind == *SITUATION_KIND_AIR);
            }
            if fall_check {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                if is_end {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                    }
                }
            }
        }
        if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
            if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32)
            || GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
                if damage_fly_reflect_speed < get_speed_x {
                    if break_to_death.get_bool() {
                        fighter.change_status(FIGHTER_STATUS_KIND_DOLLY_STAGE_DEAD.into(), false.into());
                    }
                    else {
                        fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR.into(), false.into());
                    }
                }
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL) {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                    fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());   
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_MISS_FOOT.into(), false.into());   
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
            if FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
                return 0.into();
            }
            if [hash40("damage_n_1"), hash40("damage_n_2"), hash40("damage_n_3"), hash40("damage_hi_1"), hash40("damage_hi_2"), hash40("damage_hi_3"), hash40("damage_lw_1"), hash40("damage_lw_2"), hash40("damage_lw_3")].contains(&motion_kind) {
                fighter.change_status(FIGHTER_STATUS_KIND_DOWN_SPOT.into(), false.into());   
                return 0.into();
            }
        }
    }
    else {
        if GroundModule::is_miss_foot(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_MISS_FOOT.into(), false.into());   
        }
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_Damage)]
unsafe fn status_end_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_damage_main,
            status_end_damage
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}