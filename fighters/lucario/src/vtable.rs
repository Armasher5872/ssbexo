use super::*;

const LUCARIO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc5b730; //Lucario only
const LUCARIO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc5b8f0; //Lucario only
const LUCARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc5bb60; //Lucario only
const LUCARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc5ce40; //Lucario only
const LUCARIO_VTABLE_DEATH_RESET_OFFSET: usize = 0xc5e550; //Lucario only
const LUCARIO_VTABLE_ON_ATTACK_OFFSET: usize = 0x68d7e0; //Shared
const LUCARIO_ARTICLE_INVALID_STATUS_REMOVAL_EVENT_OFFSET: usize = 0xc5d680; //Lucario only
const LUCARIO_CHECK_AURA_1_OFFSET: usize = 0xc5c010; //Lucario only
const LUCARIO_CHECK_AURA_2_OFFSET: usize = 0xc5be40; //Lucario only
const LUCARIO_HANDLE_AURA_EFFECT_SCALE_OFFSET: usize = 0xc5e6f0; //Lucario only

unsafe extern "C" fn lucario_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6AB != 0 {
        if situation_kind == *SITUATION_KIND_GROUND
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
            fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_POWER_UP_PUNCH.into(), true.into());
            return true.into();
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
            fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_DASHING_FORCE_PALM.into(), true.into());
            return true.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_START.into(), true.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn lucario_var(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    ArticleModule::remove_exist(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, ArticleOperationTarget(0));
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    WorkModule::off_flag(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_LW_INPUT);
    WorkModule::off_flag(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MAX_AURA);
    WorkModule::set_int(boma, 0, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURA_LEVEL);
    ModelModule::set_mesh_visibility(boma, Hash40::new("gamemodel"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("lucario_close_mouth"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("lucario_eye"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("lucario_openblink"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("waist_fur"), true);
    UiManager::set_lucario_meter_info(entry_id, 0);
}

unsafe extern "C" fn aura_handle(object: *mut BattleObject) -> f32 {
    let boma = (*object).module_accessor;
    let kind = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    let aura_level = WorkModule::get_int(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURA_LEVEL) as f32;
    if kind == *FIGHTER_KIND_KIRBY {
        1.0
    }
    else {
        if WorkModule::is_flag(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MAX_AURA) {
            1.9
        }
        else {
            1.0+(aura_level*0.03)
        }
    }
}

unsafe extern "C" fn lucario_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MAX_AURA) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

//Lucario Startup Initialization
#[skyline::hook(offset = LUCARIO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucario_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    lucario_var(&mut *boma);
    set_command_input_button(boma, 0x2, 2);
    set_command_input_button(boma, 0x19, 1);
    set_command_input_button(boma, 0x19, 2);
    agent.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(lucario_end_control as *const () as _));
    agent.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(lucario_check_special_command as *const () as _));
    original!()(vtable, fighter)
}

//Lucario Reset Initialization
#[skyline::hook(offset = LUCARIO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucario_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    lucario_var(&mut *boma);
    original!()(vtable, fighter)
}

//Lucario Death Initialization
#[skyline::hook(offset = LUCARIO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucario_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    lucario_var(&mut *boma);
    original!()(vtable, fighter)
}

//Lucario Once Per Fighter Frame
#[skyline::hook(offset = LUCARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn lucario_opff(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let aura_level = WorkModule::get_int(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURA_LEVEL);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    /*
    let magic_series = fighter.battle_object.magic_series();
    if [1, 93].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
    }
    if [2, 94].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
    }
    if [3, 95].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
    }
    if [5, 27, 49, 71, 96].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
    }
    if [6, 28, 50, 72, 97].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
    }
    if [7, 29, 51, 73, 98].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
    }
    if [181, 204, 227, 250, 273].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_START, false);
    }
    if [23, 45, 67, 89, 114, 133, 152, 171, 194, 217, 240, 263, 286].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
    }
    if [24, 46, 68, 90, 115, 134, 153, 172, 195, 218, 241, 264, 287].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
    }
    if [25, 47, 69, 91, 116, 135, 154, 173, 196, 219, 242, 265, 288].contains(&magic_series) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
    }
    if [26, 48, 70, 92, 117, 136, 155, 174, 197, 220, 243, 266, 289].contains(&magic_series) && !WorkModule::is_flag(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MAX_AURA) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
    }
    */
    UiManager::set_lucario_meter_info(entry_id, aura_level);
    UiManager::set_lucario_meter_enable(entry_id, true);
    original!()(vtable, fighter)
}

//Lucario Death Reset
#[skyline::hook(offset = LUCARIO_VTABLE_DEATH_RESET_OFFSET)]
pub unsafe extern "C" fn lucario_death_reset(_vtable: u64, fighter: &mut Fighter) {
    let battle_object = &mut fighter.battle_object;
    let boma = battle_object.module_accessor;
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let aura = aura_handle(battle_object);
    WorkModule::set_float(boma, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
}

//Lucario On Attack
#[skyline::hook(offset = LUCARIO_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn lucario_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let situation_kind = agent.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = agent.global_table[CMD_CAT1].get_i32();
    let aura_level = WorkModule::get_int(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURA_LEVEL);
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_POWER_UP_PUNCH {
        if aura_level < 9 {
            //fighter.battle_object.gimmick_flash();
            WorkModule::inc_int(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURA_LEVEL);
        }
        if aura_level == 9 {
            //fighter.battle_object.gimmick_flash();
            WorkModule::inc_int(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURA_LEVEL);
            //FILL_SCREEN_MODEL_COLOR(agent, 0, 10, 0.3, 0.3, 0.3, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MAX_AURA) {
        if [
            *FIGHTER_LUCARIO_STATUS_KIND_DASHING_FORCE_PALM, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_HI, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, 
            *FIGHTER_STATUS_KIND_SPECIAL_LW
        ].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_AIR {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_N != 0
                || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_F != 0
                || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_B != 0
                || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI != 0
                || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            }
            else {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                }
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                }
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                }
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                }
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
                }
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
                }
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_DASH != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, false);
                    }
                    else {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
                    }
                }
            }
        }
        if [*FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK_LANDING, *FIGHTER_LUCARIO_STATUS_KIND_POWER_UP_PUNCH].contains(&status_kind) {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_DASH != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, false);
                }
                else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
                }
            }
        }
        if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_HIGH_JUMP_KICK {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_N != 0
            || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_F != 0
            || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_B != 0
            || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI != 0
            || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_DASH != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, false);
                }
                else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
                }
            }
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_DASH != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, false);
                }
                else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
                }
            }
        }
    }
    call_original!(vtable, fighter, log)
}

//Lucario Article Invalid Status Removal Event
#[skyline::hook(offset = LUCARIO_ARTICLE_INVALID_STATUS_REMOVAL_EVENT_OFFSET)]
pub unsafe extern "C" fn lucario_article_invalid_status_removal_event(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    WorkModule::off_flag(boma, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_FORCE_AURAPOWER_ATTACK_POWER_MUL);
    if ![
        *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_REBOUND_STOP, *FIGHTER_STATUS_KIND_REBOUND, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_HI,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_LW
    ].contains(&status_kind) {
        ArticleModule::remove_exist(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(0));
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_HI, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_LW].contains(&status_kind) {
        if status_kind == *FIGHTER_STATUS_KIND_REBOUND_STOP {
            ArticleModule::generate_article_enable(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        }
    }
}

//Lucario Check Aura 1
#[skyline::hook(offset = LUCARIO_CHECK_AURA_1_OFFSET)]
unsafe extern "C" fn lucario_check_aura_1(boma: *mut BattleObjectModuleAccessor) -> f32 {
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let battle_object = get_battle_object_from_id((*boma).battle_object_id);
    aura_handle(battle_object)
}

//Lucario Check Aura 2
#[skyline::hook(offset = LUCARIO_CHECK_AURA_2_OFFSET)]
unsafe extern "C" fn lucario_check_aura_2(module: u64) -> f32 {
    let boma = &mut *(*((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let battle_object = get_battle_object_from_id(boma.battle_object_id);
    aura_handle(battle_object)
}

//Lucario Handle Aura Effect Scale
#[skyline::hook(offset = LUCARIO_HANDLE_AURA_EFFECT_SCALE_OFFSET)]
unsafe extern "C" fn lucario_handle_aura_effect_scale(_vtable: u64, fighter: &mut Fighter) {
    let battle_object = &mut fighter.battle_object;
    let object_id = battle_object.battle_object_id;
    let boma = sv_battle_object::module_accessor(object_id);
    let aura = aura_handle(battle_object);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    WorkModule::set_float(boma, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
}

pub fn install() {
    skyline::install_hooks!(
        lucario_start_initialization,
        lucario_reset_initialization,
        lucario_death_initialization,
        lucario_opff,
        lucario_death_reset,
        lucario_on_attack,
        lucario_article_invalid_status_removal_event,
        lucario_check_aura_1,
        lucario_check_aura_2,
        lucario_handle_aura_effect_scale
    );
}