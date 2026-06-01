use super::*;

unsafe extern "C" fn armstrong_special_hi_catch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_DISABLE, false, true, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_catch_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
    let get_sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.03);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.02);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y*0.4);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x*0.25);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.0);
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        LinkModule::set_model_constraint_pos_ort(capture_boma, *LINK_NO_CAPTURE, Hash40::new("rot"), Hash40::new("throw"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
        LinkModule::set_constraint_translate_offset(capture_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_catch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW {
        effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("armstrong_flame_grab"), false, true);
        armstrong_clear_charge(boma);
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_catch_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW {
        if status_kind != *FIGHTER_STATUS_KIND_CATCH_CUT {
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
            sv_module_access::_catch(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
        }
        armstrong_clear_charge(boma);
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_armstrong_costumes_acmd())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_catch_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_catch_init_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_catch_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_catch_exit_status)
    .install()
    ;
}