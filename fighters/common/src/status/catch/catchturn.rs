use super::*;

/*   GRAB STATUSES   */
//Sub Status Catch Turn, allows item grabbing from pivot grabs
#[skyline::hook(replace = L2CFighterCommon_sub_status_CatchTurn)]
unsafe extern "C" fn sub_status_catchturn(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_TURN_RUN {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_turn"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        let frame = MotionModule::frame(fighter.module_accessor);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_turn"), frame, 1.0, false, 0.0, false, false);
    }
    PostureModule::reverse_lr(fighter.module_accessor);
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY
    ];
    for x in 0..transition_terms.len() {
        WorkModule::enable_transition_term(fighter.module_accessor, transition_terms[x]);
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
}

#[skyline::hook(replace = L2CFighterCommon_status_CatchTurn_Main)]
unsafe extern "C" fn status_catchturn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let heavy_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let light_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 || FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false) {
        if cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
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
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
        if MotionModule::is_end(fighter.module_accessor) {
            if situation_kind != *SITUATION_KIND_GROUND {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_status_catchturn,
            status_catchturn_main
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}