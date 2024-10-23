use super::*;

const ROCKMAN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const ROCKMAN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x107db60; //Mega-Man only
const ROCKMAN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x107e890; //Mega-Man only
const ROCKMAN_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x107ed30; //Mega-Man only

//Megaman Startup Initialization
#[skyline::hook(offset = ROCKMAN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rockman_start_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_ROCKMAN as u32 {
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
        WorkModule::off_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGING);
        WorkModule::off_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGED);
        WorkModule::off_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_VISUAL);
        WorkModule::set_int(boma, 0, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCK_BUSTER_CHARGE_FRAME);
    }
    original!()(vtable, fighter)
}

//Megaman Reset Initialization
#[skyline::hook(offset = ROCKMAN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rockman_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::off_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGING);
    WorkModule::off_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGED);
    WorkModule::off_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_VISUAL);
    WorkModule::set_int(boma, 0, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCK_BUSTER_CHARGE_FRAME);
    original!()(vtable, fighter)
}

//Mega-Man Death Initialization
#[skyline::hook(offset = ROCKMAN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rockman_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::off_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGING);
    WorkModule::off_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGED);
    WorkModule::off_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_VISUAL);
    WorkModule::set_int(boma, 0, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCK_BUSTER_CHARGE_FRAME);
    original!()(vtable, fighter)
}

//Mega-Man Once Per Fighter Frame
#[skyline::hook(offset = ROCKMAN_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn rockman_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let status_kind = StatusModule::status_kind(boma);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    let vtable_slow = *battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0;
    let special_lw_hold_frame = WorkModule::get_int(boma, 0x100000c3);
    if vtable_slow && !StopModule::is_stop(boma) && !SlowModule::is_skip(boma) {
        if ![*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT].contains(&status_kind) && WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD) {
            WorkModule::dec_int(boma, 0x100000c3);
            if WorkModule::is_flag(boma, 0x200000e1) {
                if special_lw_hold_frame <= 0 {
                    LinkModule::send_event_nodes(boma, *LINK_NO_ARTICLE, Hash40::new_raw(0x2435e7c874), 0);
                    ArticleModule::remove(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                    FighterSpecializer_Rockman::set_leafshield(boma, false);
                }
                if ControlModule::get_button(boma) >> 1 & 1 != 0 {
                    StatusModule::change_status_request(boma, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT, true);
                }
            }
        }
    }
    if WorkModule::is_flag(boma, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGED) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_ROCKBUSTER, 0);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("rockman_chargeshot_hold"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("rockman_chargeshot_elec"), false, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_chargeshot_hold"), Hash40::new("handl"), 4, 0, 0.5, 0, 0, 0, 0.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_chargeshot_elec"), Hash40::new("havel"), 0, 0, -1.5, 0, 0, 0, 0.5, true);
    }
    original!()(vtable, fighter)
}

#[skyline::hook(offset = 0x1083bec, inline)]
unsafe extern "C" fn rockman_link_event_disable(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor;
    FighterSpecializer_Rockman::set_leafshield(boma, false);
}

#[skyline::hook(offset = 0x10838e0, inline)]
unsafe extern "C" fn rockman_link_event_enable(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor;
    FighterSpecializer_Rockman::set_leafshield(boma, true);
}

#[skyline::hook(replace = FighterSpecializer_Rockman::set_leafshield)]
unsafe extern "C" fn set_leafshield(boma: *mut BattleObjectModuleAccessor, set_shield: bool) {
    WorkModule::set_flag(boma, set_shield, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD);
    WorkModule::set_flag(boma, set_shield, 0x200000e1);
    if !set_shield {
        WorkModule::set_int(boma, 0, 0x100000c3);
        WorkModule::unable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
        WorkModule::unable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
        WorkModule::unable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
        WorkModule::unable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::unable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::unable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::unable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    }
    else {
        let hold_frame = WorkModule::get_param_int(boma, hash40("param_special_lw"), hash40("hold_frame"));
        WorkModule::set_int(boma, hold_frame, 0x100000c3);
        WorkModule::enable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
        WorkModule::enable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
        WorkModule::enable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
        WorkModule::enable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::enable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::enable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::enable_transition_term_forbid_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x107eaa4).data(0x1400001Eu32); //The following patch disables the original function that forced Mega Man into Leaf Shield Throw. This is so that the custom function can be used. This offset is located in Mega-Man's OPFF
    let _ = skyline::patching::Patch::in_text(0x107ff6c).data(0x14000007u32); //The following patch disables the removal of Leaf Shield if Mega Man enters certain statuses
    skyline::install_hooks!(
        rockman_start_initialization,
        rockman_reset_initialization,
        rockman_death_initialization,
        rockman_opff,
        rockman_link_event_disable,
        rockman_link_event_enable,
        set_leafshield
    );
}