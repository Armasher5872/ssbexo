/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Dash)]
unsafe fn status_pre_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_DashCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_DASH, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, 0, (*FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32, 0, 0);
    0.into()
}

//Dash Common
#[skyline::hook(replace = L2CFighterCommon_status_DashCommon)]
unsafe fn status_dash_common(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    /* The transition terms from CONT_ATTACK_DASH to CONT_ESCAPE_B are the transition terms that are present in the original function. Any transition terms after CONT_ESCAPE_B are added to the list, being Platform Drops, Crouching, and Taunts */
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4, *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW
    ];
    //Effectively enables the ability to transition into the listed status kinds
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    for x in 0..transition_terms.len() {
        WorkModule::enable_transition_term(boma, transition_terms[x]);
    }
    //Calls and then sets values regarding dash dancing windows and when you can attack out of initial dash
    let turn_dash_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("turn_dash_frame"));
    let retry_turn_dash_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("retry_turn_dash_frame"));
    let dash_enable_attack_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("dash_enable_attack_frame"));
    WorkModule::set_int(boma, turn_dash_frame, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
    WorkModule::set_int(boma, retry_turn_dash_frame, *FIGHTER_STATUS_DASH_WORK_INT_RETRY_TURN_DASH_FRAME);
    WorkModule::set_int(boma, dash_enable_attack_frame, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_RUN_BRAKE {
        let run_brake_attack_escape_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("run_brake_attack_escape_frame"));
        WorkModule::set_int(boma, run_brake_attack_escape_frame-fighter.global_table[0x25].get_i32(), *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME);
        if 0 < WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME) {
            //Disables the ability to transition into the following statuses if your invalid_attack_escape_frame is less than 0 if your previous status kind was run_brake
            let unable_transition_terms = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE, 
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT
            ];
            for x in 0..unable_transition_terms.len() {
                WorkModule::unable_transition_term(boma, unable_transition_terms[x]);
            }
        }
    }
}

//Dash Main Common
#[skyline::hook(replace = L2CFighterCommon_status_Dash_Main_common)]
unsafe fn status_dash_main_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    /* Any new additions to this hook will be denoted by: "New Addition", otherwise it's the vanilla script */
    let const_stick_x = fighter.global_table[STICK_X].get_f32(); 
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = const_stick_x * lr;
    if fighter.global_table[DASH_COMMON_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[DASH_COMMON_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 1.into();
    }
    //Changes the status kind to fall if you're in the air
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    //Enables canceling
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    //Allows transition into Shorthop Attacks
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 1.into();
    }
    let mut can_s4 = true;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        can_s4 = stick_x < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x206138766c)
    }
    //Changes the status kind to item throw if you perform the input for it, and you're able to throw items
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) && can_s4 {
        let mut throw = false;
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            throw = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            let item_bullet = ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0);
            if item_bullet <= 0 {
                throw = true;
            }
            else {
                throw = false;
            }
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }
    //Changes the status kind to the type of item throw if you perform the input for it
    let item_lua_stack_throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let item_lua_stack_no_throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW); sv_module_access::item(fighter.lua_state_agent); !fighter.pop_lua_stack(1).get_bool()};
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH) && item_lua_stack_throw && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE) && ItemModule::is_have_item(fighter.module_accessor, 0) && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && item_lua_stack_no_throw {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH) && ItemModule::is_have_item(fighter.module_accessor, 0) && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && item_lua_stack_no_throw {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), false.into());
        return 1.into();
    }
    //Changes the status kind to pivot grab if you're able to pivot grab
    let catch_turn_condition = {let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x")); stick_x <= turn_run_stick_x};
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) && catch_turn_condition && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        return 1.into();
    }
    //Changes the status kind to dash grab if you're able to dash grab
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
        return 1.into();
    }
    //Allows transition into command input specials
    if fighter.sub_transition_group_check_special_command().get_bool() {
        return true.into();
    }
    //Allows transition into grounded specials
    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return true.into();
    }
    //Allows transition into the special flag use status
    if fighter.sub_transition_specialflag_hoist().get_bool() {
        return true.into();
    }
    //Changes the status kind to item forward smash if you're able to item forward smash
    let item_lua_stack_swing = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) 
    && item_lua_stack_swing 
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0 
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
        return 1.into();
    }
    //If you're able to shoot an item, transition into the item shoot status, otherwise transition into the item throw status
    let item_lua_stack_shoot = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) 
    && item_lua_stack_shoot 
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        if ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) > 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
            return 1.into();
        }
    }
    if fighter.global_table[CHECK_ATTACK_S4_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_S4_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 1.into();
    }
    //Changes the status kind to forward smash if you're able to forward smash
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START)
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
        return 1.into();
    }
    //Changes the status kind to item dash attack if you're able to item dash attack
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH) && item_lua_stack_swing && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_DASH.into(), true.into());
        return 1.into();
    }
    //Changes the status kind to dash attack if you're able to dash attack
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return true.into();
    }
    //Goes through a variety of checks to see if you transition into the heavy pickup status or light pickup status
    let heavy_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let light_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME) 
    && (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 || FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false)) {
        if fighter.global_table[CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 
        && heavy_item
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
            return true.into();
        }
        if ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH)
        && light_item {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
            return true.into();
        }
    }
    //Changes the status kind to turn dash if you're able to turn dash
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) 
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
        return 1.into();
    }
    let foxtrot_check = {let frame = MotionModule::frame(fighter.module_accessor); let re_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("re_dash_frame")) as f32; re_dash_frame <= frame};
    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 && foxtrot_check {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 1.into();
    }
    if param_1.get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_1.get_ptr());
        if callable(fighter).get_bool() {
            return 1.into();
        }
    }
    //Changes the status kind to run if your stick_x is greater than or equal to the stick_x required to run, otherwise change the status kind to walk
    let run_stick_x_check = {let run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x")); run_stick_x <= stick_x};
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) && run_stick_x_check {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN) {
            fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), true.into());
        }
        return 1.into();
    }
    //Changes the status kind to walk brake if your friction is low enough
    if GroundModule::get_down_friction(fighter.module_accessor) < 1.0 && FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
        fighter.change_status(FIGHTER_STATUS_KIND_WALK_BRAKE.into(), false.into());
        return 1.into();
    }
    //Changes the status kind to wall bump if you're in range, otherwise change to wait
    if !MotionModule::is_end(fighter.module_accessor) {
        if fighter.sub_ground_check_stop_wall().get_bool() {
            return 1.into();
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    /* START OF NEW ADDITION */
    //Allows crouch out of initial dash
    if fighter.down_input() && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR && stick_x < 0.75 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT) && fighter.sub_check_command_squat().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT.into(), true.into());
            return 1.into();
        }
    }
    //Allows platform drops out of initial dash
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_flick_y")) as i32;
    if GroundModule::is_passable_ground(fighter.module_accessor) && fighter.global_table[STICK_Y].get_f32() < pass_stick_y && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return 1.into();
    }
    //Allows taunts out of initial dash
    let notify_taunt_hash = {fighter.clear_lua_stack(); fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be)); smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0) 
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0) 
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 
    || fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0)) 
    && notify_taunt_hash {
        fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
        return 1.into();
    }
    //Captain Falcon's Dash Grab Speed
    if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_CAPTAIN {
        WorkModule::set_float(fighter.module_accessor, 1.933, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    }
    /* END OF NEW ADDITION */
    0.into()
}

//Dash Back Main
pub unsafe extern "C" fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Due to the fighting game characters having a different status for their dashes, this needs to be be implemented alongside the normal dash status script to allow for the new features to apply for them
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("dash_b"), 0.0, 1.0, false, 0.0, false, false);
    fighter.status_DashCommon();
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    GroundModule::set_reverse_direction(fighter.module_accessor, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_dashback_main_loop as *const () as _))
}

//Dash Back Loop
unsafe extern "C" fn fgc_dashback_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    /* Any new additions to this hook will be denoted by: "New Addition", otherwise it's the vanilla script */
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let const_stick_x = fighter.global_table[STICK_X].get_f32(); 
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let flick_y = fighter.global_table[FLICK_Y].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = const_stick_x * lr;
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), 1.into());
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) {
        let mut throw = false;
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            throw = pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            let item_bullet = ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0);
            if item_bullet <= 0 {
                throw = true;
            }
            else {
                throw = false;
            }
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }
    let item_lua_stack_no_throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW); sv_module_access::item(fighter.lua_state_agent); !fighter.pop_lua_stack(1).get_bool()};
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE)
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 
    && item_lua_stack_no_throw {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            return 1.into();
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE) {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_special_command().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return true.into();
    }
    if fighter.sub_transition_specialflag_hoist().get_bool() {
        return true.into();
    }
    let item_lua_stack_swing = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) 
    && item_lua_stack_swing 
    && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
        return 1.into();
    }
    let item_lua_stack_shoot = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) 
    && item_lua_stack_shoot 
    && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        if ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START)
    && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return true.into();
    }
    let heavy_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let light_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME)
    && (cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false)) {
        if cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 
        && heavy_item 
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
            return true.into();
        }
        if ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH)
        && light_item {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
            return true.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
    }
    let foxtrot_check = {let frame = MotionModule::frame(fighter.module_accessor); let re_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("re_dash_frame")) as f32; re_dash_frame <= frame};
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 && foxtrot_check {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
        return 1.into();
    }
    /* START OF NEW ADDITION */
    //Allows crouch out of back dash
    if fighter.down_input() && situation_kind != *SITUATION_KIND_AIR && stick_x < 0.75 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT) && fighter.sub_check_command_squat().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT.into(), true.into());
            return 1.into();
        }
    }
    //Allows platform drops out of back dash
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_flick_y")) as i32;
    if GroundModule::is_passable_ground(fighter.module_accessor) && stick_y < pass_stick_y && flick_y < pass_flick_y {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return 1.into();
    }
    //Allows taunts out of back dash
    let notify_taunt_hash = {fighter.clear_lua_stack(); fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be)); smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0) 
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0) 
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0)) 
    && notify_taunt_hash {
        fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
        return 1.into();
    }
    /* END OF NEW ADDITION */
    let run_stick_x_check = {let run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x")); run_stick_x <= (stick_x * -1.0)};
    if !fighter.sub_transition_group_check_ground_jump().get_bool() {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) && run_stick_x_check 
        && cmd_cat1 & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | 
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW
        ) == 0 {
            let kind;
            if fighter_kind == *FIGHTER_KIND_DOLLY {
                kind = FIGHTER_DOLLY_STATUS_KIND_TURN_RUN_BACK;
            }
            else if fighter_kind == *FIGHTER_KIND_DEMON {
                kind = FIGHTER_DEMON_STATUS_KIND_TURN_RUN_BACK;
            }
            else  {
                kind = FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK;
            }
            fighter.change_status(kind.into(), false.into());
            return 1.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            if GroundModule::get_down_friction(fighter.module_accessor) < 1.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_RUN_BRAKE.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            1.into()
        }
        else {
            0.into()
        }
    }
    else {
        1.into()
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_dash,
            status_dash_common,
            status_dash_main_common
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}