use super::*;

pub unsafe extern "C" fn gaogaen_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_ENABLE_JAB_GRAB);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CAN_CANCEL);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCELED);
}

pub unsafe extern "C" fn fun_710001a8c0(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let fall_hit_object_id = WorkModule::get_int(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_INT_FALL_HIT_OBJECT_ID) as u32;
    if fall_hit_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        if lua_bind::BattleObjectManager::is_active_find_battle_object(singletons::BattleObjectManager(), fall_hit_object_id) {
            let fall_boma = sv_battle_object::module_accessor(fall_hit_object_id);
            let fall_status_kind = StatusModule::status_kind(fall_boma);
            let transition_terms = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
            ];
            if [*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&fall_status_kind) {
                for x in transition_terms.iter() {
                    WorkModule::enable_transition_term(boma, *x);
                }
            }
        }
    }
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_INT_FALL_HIT_OBJECT_ID);
}

pub unsafe extern "C" fn fun_7100013e50(fighter: &mut L2CFighterCommon, bool_check: L2CValue) {
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let mut get_sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_lw_start_x_mul = WorkModule::get_param_float(boma, hash40("param_special_lw"), hash40("special_lw_start_x_mul"));
    let special_air_lw_speed_y_mul = WorkModule::get_param_float(boma, hash40("param_special_lw"), hash40("special_air_lw_speed_y_mul"));
    let special_air_lw_hit_accel_y = WorkModule::get_param_float(boma, hash40("param_special_lw"), hash40("special_air_lw_hit_accel_y"));
    KineticModule::change_kinetic(boma, if situation_kind != *SITUATION_KIND_GROUND {*FIGHTER_KINETIC_TYPE_FALL} else {*FIGHTER_KINETIC_TYPE_GROUND_STOP});
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if bool_check.get_bool() {
            get_sum_speed_x = get_sum_speed_x*special_lw_start_x_mul;
        }
        if situation_kind != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, get_sum_speed_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            if bool_check.get_bool() {
                if !WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_INVALID_SPECIAL_AIR_LW_SPPED_Y) {
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y*special_air_lw_speed_y_mul);
                    WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_INVALID_SPECIAL_AIR_LW_SPPED_Y);
                }
            }
            else {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
            }
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, boma);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, get_sum_speed_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        }
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, boma);
    }
    else {
        if situation_kind != *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, get_sum_speed_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            if bool_check.get_bool() {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            }
            else {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
            }
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_air_lw_hit_accel_y);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, boma);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, get_sum_speed_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        }
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if !bool_check.get_bool() {
            if status_kind_interrupt == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_TURN {
                sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, -1.0);
            }
        }
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, boma);
    }
}