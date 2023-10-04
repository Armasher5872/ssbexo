use super::*;

/*   GRAB STATUSES   */
//Sub Status Catch, allows item grabbing from standing grabs
#[skyline::hook(replace = L2CFighterCommon_sub_status_Catch)]
unsafe fn sub_status_catch(fighter: &mut L2CFighterCommon) {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
    let transition_terms = [*FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY];
    for x in 0..transition_terms.len() {
        WorkModule::enable_transition_term(fighter.module_accessor, transition_terms[x]);
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_Catch_Main)]
unsafe fn status_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*START OF NEW ADDITIONS*/
    //Goes through a variety of checks to see if you transition into the heavy pickup status or light pickup status
    let heavy_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let light_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false) {
        if cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY)
            && ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_HEAVY as u64
            && heavy_item 
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
                return 1.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT)
            && ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
            && light_item
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), false.into());
                return 1.into();
            }
        }
    }
    /*END OF NEW ADDITIONS*/
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_status_catch,
            status_catch_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}