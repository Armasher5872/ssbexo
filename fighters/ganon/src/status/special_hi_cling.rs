use super::*;

unsafe extern "C" fn ganon_special_hi_cling_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let pos = PostureModule::pos(boma);
    let effect = EffectModule::req_follow(boma, Hash40::new("ganon_entry"), Hash40::new("hip"), &Vector3f{x: (*pos).x, y: (*pos).y, z: 0.0}, &Vector3f::zero(), 0.5, true, 0, 0, 0, 0, 0, true, true);
    if !WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_STARTED_GROUNDED) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    }
    EffectModule::set_rgb(boma, effect as u32, 1.0, 0.0, 0.0);
    WorkModule::set_int(boma, effect as i32, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_cling_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW {
        effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("ganon_raijin"), false, true);
    }
    EFFECT_OFF_KIND(fighter, Hash40::new("ganon_entry"), true, true);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
    WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_STARTED_GROUNDED);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_cling_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW {
        if status_kind != *FIGHTER_STATUS_KIND_CATCH_CUT {
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
            sv_module_access::_catch(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
        }
    }
    EFFECT_OFF_KIND(fighter, Hash40::new("ganon_entry"), true, true);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
    WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_STARTED_GROUNDED);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, ganon_special_hi_cling_init_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, ganon_special_hi_cling_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, ganon_special_hi_cling_exit_status)
    .install()
    ;
}