use super::*;

const DOLLY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x9708f0; //Terry only
const DOLLY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x970900; //Terry only
const DOLLY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x970e40; //Terry only
const DOLLY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x971c80; //Terry only

unsafe extern "C" fn dolly_check_special_hi_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_cat_flag(Cat4::SpecialHi2Command) && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].clone()).get_bool() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
        }
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn dolly_check_super_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let cancel_statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&prev_status_kind);
    let opplr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    WorkModule::set_int(fighter.module_accessor, fighter.global_table[CMD_CAT4].get_i32(), *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_CAT4_SPECIAL_COMMAND);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
        if fighter.is_cat_flag(Cat4::SuperSpecial2Command)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
            || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
            && !cancel_statuses {
                WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
            return true.into();
        }
        if fighter.is_cat_flag(Cat4::SuperSpecialCommand)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
            || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
            && !cancel_statuses {
                WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
            return true.into();
        }
        if fighter.is_cat_flag(Cat4::SuperSpecial2RCommand)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
            || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            && !cancel_statuses {
                WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            }
            if opplr != 0.0 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
            return true.into();
        }
        if fighter.is_cat_flag(Cat4::SuperSpecialRCommand)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
            || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            && !cancel_statuses {
                WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            }
            if opplr != 0.0 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

unsafe extern "C" fn dolly_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let cancel_statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&prev_status_kind);
    if dolly_check_super_special_command(fighter).get_bool() {
        return true.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && dolly_check_special_hi_command(fighter).get_bool() {
        return true.into();
    }
    if fighter.is_cat_flag(Cat4::SpecialHiCommand)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_LW_UNIQ].clone()).get_bool() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && !cancel_statuses {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
        }
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND.into(), true.into());
        return true.into();
    }
    if fighter.is_cat_flag(Cat4::SpecialSCommand)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && !cancel_statuses {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
        }
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
        return true.into();
    }
    if fighter.is_cat_flag(Cat4::SpecialNCommand)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && !cancel_statuses {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
        }
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND.into(), true.into());
        return true.into();
    }
    if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL))
    && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK))
    && !cancel_statuses {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), false.into());
    }
    false.into()
}

//Terry Startup Initialization
#[skyline::hook(offset = DOLLY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dolly_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let lua_module_fighter = get_fighter_common_from_accessor(&mut *boma);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
    lua_module_fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(dolly_check_special_command as *const () as _));
}

//Terry Reset Initialization
#[skyline::hook(offset = DOLLY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dolly_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
    original!()(vtable, fighter)
}

//Terry Death Initialization
#[skyline::hook(offset = DOLLY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dolly_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
    original!()(vtable, fighter)
}

//Terry Once Per Fighter Frame
#[skyline::hook(offset = DOLLY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn dolly_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let frame = MotionModule::frame(boma);
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let situation_kind = StatusModule::situation_kind(boma);
    let cancel_statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&prev_status_kind);
    if WorkModule::is_flag(boma, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED)
    && situation_kind != *SITUATION_KIND_AIR {
        if [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND].contains(&status_kind)
        && frame >= 9.0
        && !cancel_statuses {
            WorkModule::set_flag(boma, false, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND
        && frame >= 11.0
        && !cancel_statuses {
            WorkModule::set_flag(boma, false, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND
        && frame >= 5.0
        && !cancel_statuses {
            WorkModule::set_flag(boma, false, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL
        && frame >= 13.0
        && !cancel_statuses {
            WorkModule::set_flag(boma, false, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        dolly_start_initialization,
        dolly_reset_initialization,
        dolly_death_initialization,
        dolly_opff
    );
}