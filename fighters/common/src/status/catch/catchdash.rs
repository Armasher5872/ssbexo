use super::*;

/*   GRAB STATUSES   */
//Sub Status Catch Dash, allows item grabbing from dash grabs
#[skyline::hook(replace = L2CFighterCommon_sub_status_CatchDash)]
unsafe extern "C" fn sub_status_catchdash(fighter: &mut L2CFighterCommon) {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_dash"), 0.0, 1.0, false, 0.0, false, false);
    let catch_turn_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_turn_frame"));
    WorkModule::set_int(fighter.module_accessor, catch_turn_frame, *FIGHTER_STATUS_CATCH_DASH_WORK_INT_CATCH_TURN_FRAME);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
    let transition_terms = [*FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH];
    for x in 0..transition_terms.len() {
        WorkModule::enable_transition_term(fighter.module_accessor, transition_terms[x]);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.CatchDashUniq(L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_CatchDashUniq as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_CatchDash_Main)]
unsafe extern "C" fn status_catchdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Goes through a variety of checks to see if you transition into the heavy pickup status or light pickup status
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    let heavy_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let light_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 || FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false)) {
        if fighter.global_table[CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
            && ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_HEAVY as u64
            && heavy_item 
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
                return true.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH)
            && ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
            && light_item
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
                return true.into();
            }
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
        if stick_x*lr <= turn_run_stick_x {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
        if MotionModule::is_end(fighter.module_accessor) {
            if situation_kind != *SITUATION_KIND_GROUND {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_status_catchdash,
            status_catchdash_main
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}