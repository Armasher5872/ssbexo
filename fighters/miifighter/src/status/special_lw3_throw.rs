use super::*;

unsafe extern "C" fn miifighter_special_lw3_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
    if situation_kind == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    0.into()
}

unsafe extern "C" fn miifighter_special_lw3_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_node_object_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    let counter_attack_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
    let lw3_attack_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_attack_mul"));
    let lw3_attack_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_attack_max"));
    let lw3_attack_max_for_enemy = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_attack_max_for_enemy"));
    let lw3_attack_power_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_attack_power_limit"));
    let mut attack_power = counter_attack_power*lw3_attack_mul;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw3_throw_toss"), L2CValue::Hash40s("special_air_lw3_throw_toss"), false.into());
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CATCH_SET_CATCH);
        sv_module_access::_catch(fighter.lua_state_agent);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        if attack_power < lw3_attack_power_limit {
            attack_power = 0.0;
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_IS_ATTACK_ENEMY) {
            if lw3_attack_max_for_enemy < attack_power {
                attack_power = lw3_attack_max_for_enemy;
            }
        }
        else {
            if lw3_attack_max < attack_power {
                attack_power = lw3_attack_max
            }
        }
        WorkModule::set_float(fighter.module_accessor, attack_power, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
        WorkModule::set_int(fighter.module_accessor, get_node_object_id as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw3_throw"), L2CValue::Hash40s("special_air_lw3_throw"), false.into());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(miifighter_special_lw3_throw_main_loop as *const () as _))
}

unsafe extern "C" fn miifighter_special_lw3_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let attack_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
    let counter_throw_object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING) {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw3_throw_toss"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw3_throw"), -1.0, 1.0, 0.0, false, false);
                }
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK) {
        if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                let init_life = WorkModule::get_int(counter_throw_boma, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
                WorkModule::set_int(counter_throw_boma, init_life*2, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                GroundModule::set_ignore_boss(counter_throw_boma, true);
                GroundModule::set_passable_check(counter_throw_boma, false);
                GroundModule::set_collidable(counter_throw_boma, false);
                JostleModule::set_status(counter_throw_boma, false);
                if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                    LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
                }
                if !LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                    VisibilityModule::set_whole(counter_throw_boma, true);
                    LinkModule::link(counter_throw_boma, *LINK_NO_ARTICLE, (*fighter.module_accessor).battle_object_id);
                    LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *LINK_NO_ARTICLE, Hash40::new("rot"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                    LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                }
            }
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                let init_life = WorkModule::get_int(counter_throw_boma, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
                WorkModule::set_int(counter_throw_boma, init_life*2, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                GroundModule::set_ignore_boss(counter_throw_boma, true);
                GroundModule::set_passable_check(counter_throw_boma, false);
                GroundModule::set_collidable(counter_throw_boma, false);
                JostleModule::set_status(counter_throw_boma, false);
                if LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                    LinkModule::unlink(counter_throw_boma, *ITEM_LINK_NO_HAVE);
                }
                if !LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                    VisibilityModule::set_whole(counter_throw_boma, true);
                    LinkModule::link(counter_throw_boma, *ITEM_LINK_NO_HAVE, (*fighter.module_accessor).battle_object_id);
                    LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *ITEM_LINK_NO_HAVE, Hash40::new("rot"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                    LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                }
            }
        }
    }
    else {
        if 0.0 < attack_power {
            AttackModule::set_power(fighter.module_accessor, 0, attack_power, true);
        }
        if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                    LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
                }
            }
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                if LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                    LinkModule::unlink(counter_throw_boma, *ITEM_LINK_NO_HAVE);
                }
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn miifighter_special_lw3_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let counter_throw_object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    CatchModule::catch_cut(fighter.module_accessor, false, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
    if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    }
    0.into()
}

unsafe extern "C" fn miifighter_special_lw3_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let counter_throw_object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
    if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    }
    0.into()
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, miifighter_special_lw3_throw_init_status)
    .status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, miifighter_special_lw3_throw_main_status)
    .status(End, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, miifighter_special_lw3_throw_end_status)
    .status(Exit, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, miifighter_special_lw3_throw_exit_status)
    .install()
    ;
}