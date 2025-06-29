use super::*;

const CLOUD_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x8dacd0; //Cloud only
const CLOUD_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x8db3b0; //Cloud only
const CLOUD_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x8db780; //Cloud only
const CLOUD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x68d670; //Shared
const CLOUD_VTABLE_SHIELD_ATTACK_DETECTION_EVENT_OFFSET: usize = 0x68d8c0; //Shared
const CLOUD_VTABLE_SHIELD_ATTACK_TRANSITION_EVENT_OFFSET: usize = 0x68d8d0; //Shared
const CLOUD_LIMIT_MANAGER_OFFSET: usize = 0x8dc160; //Cloud only

unsafe extern "C" fn cloud_var(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISH_COUNTER);
    WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_DECIDE);
    WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_CHOSEN);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_HANDLE);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ATTACK_ANGLE);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    UiManager::set_cloud_meter_info(entry_id, 0);
}

unsafe extern "C" fn cloud_check_ground_guard_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) && fighter.sub_check_command_guard().get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
            fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_GUARD_ON.into(), true.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn cloud_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

//Cloud Startup Initialization
#[skyline::hook(offset = CLOUD_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn cloud_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let shield_data = ShieldDataResource::new(0.0, 9.5, 3.0, 0.0, 9.5, 3.0, 13.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource::new(shield_datas, 1, 0, false, false, false);
    common_initialization_variable_reset(&mut *boma);
    cloud_var(&mut *boma);
    add_shield_group(boma, resource, *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(cloud_end_control as *const () as _));
    agent.global_table[CHECK_GROUND_GUARD_UNIQ].assign(&L2CValue::Ptr(cloud_check_ground_guard_uniq as *const () as _));
    original!()(vtable, fighter)
}

//Cloud Reset Initialization
#[skyline::hook(offset = CLOUD_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn cloud_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    cloud_var(&mut *boma);
    ShieldModule::set_target_property(boma, *COLLISION_PROPERTY_NORMAL, *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    original!()(vtable, fighter)
}

//Cloud Death Initialization
#[skyline::hook(offset = CLOUD_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn cloud_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let limit_level = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISH_COUNTER);
    WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_DECIDE);
    WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_CHOSEN);
    match limit_level {
        0 => {
            WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
        },
        1 => {
            WorkModule::set_int(boma, limit_level-1, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
        },
        _ => {
            WorkModule::set_int(boma, limit_level-2, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
        }
    }
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_HANDLE);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ATTACK_ANGLE);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    UiManager::set_cloud_meter_info(entry_id, 0);
    original!()(vtable, fighter)
}

//Cloud Once Per Fighter Frame
#[skyline::hook(offset = CLOUD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn cloud_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_CLOUD as u32 {
        let boma = fighter.battle_object.module_accessor;
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        let limit_level = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
        if limit_level > 0 {
            UiManager::set_cloud_meter_enable(entry_id, true);
            UiManager::set_cloud_meter_info(entry_id, limit_level-1);
        }
        else {
            UiManager::set_cloud_meter_enable(entry_id, false);
            UiManager::set_cloud_meter_info(entry_id, 0);
        }
        cloud_training_mode_features(boma);
    }
    original!()(vtable, fighter)
}

//Cloud Shield Attack Detection Event
#[skyline::hook(offset = CLOUD_VTABLE_SHIELD_ATTACK_DETECTION_EVENT_OFFSET)]
unsafe extern "C" fn cloud_shield_attack_detection_event(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_CLOUD as u32 {
        let boma = fighter.battle_object.module_accessor;
        let collision_log = *(log as *const u64).add(0x10/0x8);
        let collision_log = collision_log as *const CollisionLog;
        let opponent_id = (*collision_log).opponent_battle_object_id;
        let status_kind = StatusModule::status_kind(boma);
        if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 || opponent_id != 0 {
            if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                if [*FIGHTER_CLOUD_STATUS_KIND_GUARD_ON, *FIGHTER_CLOUD_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
                    WorkModule::on_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISH_COUNTER);
                }   
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Cloud Shield Attack Transition Event
#[skyline::hook(offset = CLOUD_VTABLE_SHIELD_ATTACK_TRANSITION_EVENT_OFFSET)]
unsafe extern "C" fn cloud_shield_attack_transition_event(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_CLOUD as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        let pos_x = PostureModule::pos_x(boma);
        let collision_log = *(log as *const u64).add(0x10/0x8);
        let collision_log = collision_log as *const CollisionLog;
        let opponent_id = (*collision_log).opponent_battle_object_id;
        let mut facing = 1.0;
        if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISH_COUNTER) {
            if [*FIGHTER_CLOUD_STATUS_KIND_GUARD_ON, *FIGHTER_CLOUD_STATUS_KIND_GUARD].contains(&status_kind) {
                if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 || opponent_id != 0 {
                    let opponent_boma = sv_battle_object::module_accessor(opponent_id as u32);
                    let opponent_pos_x = PostureModule::pos_x(opponent_boma);
                    if opponent_pos_x <= pos_x {
                        facing = -1.0;
                    }
                }
                PostureModule::set_lr(boma, facing);
                PostureModule::update_rot_y_lr(boma);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);   
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Cloud Limit Manager
#[skyline::hook(offset = CLOUD_LIMIT_MANAGER_OFFSET)]
unsafe extern "C" fn cloud_limit_manager(limit: f32, boma: *mut BattleObjectModuleAccessor, param_3: u64) {
    let get_team_color = FighterUtil::get_team_color(boma) as i32;
    let limit_gauge = WorkModule::get_float(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    let limit_gauge_effect = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_GAUGE_EFFECT);
    let limit_level = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    let limit_gauge_offset_y = WorkModule::get_param_float(boma, hash40("param_special_lw"), hash40("limit_gauge_offset_y"));
    let mut gauge = limit_gauge+limit;
    if !WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
        if 100.0 <= gauge {
            if limit_level == 4 {
                EffectModule::req_follow(boma, Hash40::new("cloud_limitbreak_aura"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 2.0, false, *EFFECT_SUB_ATTRIBUTE_SYNC_INIT_POS as u32, 0, -1, 0, 0, false, false);
                EffectModule::req_common(boma, Hash40::new("cloud_limitbreak"), 0.0);
                EffectModule::req_follow(boma, Hash40::new_raw(0x16102a334b), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, -1, 0, 0, false, false);
                if limit_gauge_effect != 0 {
                    EffectModule::detach(boma, limit_gauge_effect as u32, 0);
                    EffectModule::req_follow(boma, Hash40::new_raw(0x14d013ba16), Hash40::new("top"), &Vector3f{x: 0.0, y: limit_gauge_offset_y, z: 0.0}, &Vector3f{x: 0.0, y: limit_gauge_offset_y, z: 0.0}, 1.0, true, 0, 0, get_team_color, 0, 0, false, false);
                    EffectModule::set_rot(boma, limit_gauge_effect as u32, &Vector3f{x: 0.0, y: limit_gauge_offset_y, z: 0.0});
                    WorkModule::set_int64(boma, limit_gauge_effect as i64, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_GAUGE_EFFECT);
                }
                SoundModule::play_se(boma, Hash40::new("se_cloud_special_l03"), true, false, false, false, enSEType(0));
                FighterUtil::flash_eye_info(boma);
                WorkModule::on_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
                WorkModule::set_int(boma, i32::MAX, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_CLEAR_FRAME);
            }
            else {
                WorkModule::inc_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
                WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
                WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
                WorkModule::set_float(boma, 0.0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
                gauge = 0.0;
                EffectModule::set_frame(boma, limit_gauge_effect as u32, 0.0);
                FighterUtil::flash_eye_info(boma);
                SoundModule::play_se(boma, Hash40::new("se_cloud_special_l03"), true, false, false, false, enSEType(0));
            }
        }
        else {
            if gauge <= 0.0 {
                gauge = 0.0;
            }
            if limit_gauge_effect != 0 {
                EffectModule::set_frame(boma, limit_gauge_effect as u32, gauge);
            }
        }
        WorkModule::set_float(boma, gauge, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
        if param_3 != 0 {
            WorkModule::on_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_GAUGE_CHARGE);
        }
    }
}

pub fn install() {
    //The following nop disables Cloud's Waza Customize being enabled for Neutral Special, thusly preventing Limit Break Neutral Special from being used
    let _ = skyline::patching::Patch::in_text(0x8dd868).nop();
	skyline::install_hooks!(
        cloud_start_initialization,
        cloud_reset_initialization,
        cloud_death_initialization,
        cloud_opff,
        cloud_shield_attack_detection_event,
        cloud_shield_attack_transition_event,
        cloud_limit_manager
    );
}