use super::*;

const SNAKE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x11b5710;
const SNAKE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x11b5720;
const SNAKE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x11b59f0;

//Snake Startup Initialization
#[skyline::hook(offset = SNAKE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn snake_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
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
    original!()(vtable, fighter)
}

//Snake Reset Initialization
#[skyline::hook(offset = SNAKE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn snake_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
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
    original!()(vtable, fighter)
}

//Snake Once Per Fighter Frame
#[skyline::hook(offset = SNAKE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn snake_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let stick_y = ControlModule::get_stick_y(boma);
    let kind = fighter.battle_object.kind as i32;
    let heavy_item = [*ITEM_KIND_BARREL, *ITEM_KIND_BOX, *ITEM_KIND_CARRIERBOX, *ITEM_KIND_KUSUDAMA, *ITEM_KIND_SNAKECBOX, *ITEM_KIND_WARIOBIKE].contains(&kind);
    let light_item = [
        *ITEM_KIND_BACKSHIELD, *ITEM_KIND_BADGE, *ITEM_KIND_BANANA, *ITEM_KIND_BANANAGUN, *ITEM_KIND_BANANAGUN_EMPTY, *ITEM_KIND_BEAMSWORD, *ITEM_KIND_BEETLE, *ITEM_KIND_BLACKBALL, *ITEM_KIND_BOMBCHU, *ITEM_KIND_BOMBER, *ITEM_KIND_BOMBHEI, *ITEM_KIND_BOOK,
        *ITEM_KIND_BOOMERANG, *ITEM_KIND_BOSSGALAGA, *ITEM_KIND_BUDDYBOMB, *ITEM_KIND_BUMPER, *ITEM_KIND_CAPSULE, *ITEM_KIND_CHEWING, *ITEM_KIND_CHICKEN, *ITEM_KIND_CLUB, *ITEM_KIND_CROSSBOMB, *ITEM_KIND_CURRY, *ITEM_KIND_DEATHSCYTHE, *ITEM_KIND_DEKU,
        *ITEM_KIND_DIDDYPEANUTS, *ITEM_KIND_DOSEISAN, *ITEM_KIND_DRAGOON, *ITEM_KIND_DRILL, *ITEM_KIND_DRILL_EMPTY, *ITEM_KIND_EXPLOSIONBOMB, *ITEM_KIND_FAIRYBOTTLE, *ITEM_KIND_FINALDAISY, *ITEM_KIND_FINALPEACH, *ITEM_KIND_FIREBAR, *ITEM_KIND_FIREFLOWER,
        *ITEM_KIND_FREEZER, *ITEM_KIND_GOLDENHAMMER, *ITEM_KIND_GREENSHELL, *ITEM_KIND_HAMMER, *ITEM_KIND_HEALBALL, *ITEM_KIND_HEART, *ITEM_KIND_HOMERUNBAT, *ITEM_KIND_HOMERUNBATHR, *ITEM_KIND_HONEYCOMB, *ITEM_KIND_KILLER, *ITEM_KIND_KILLEREYE,
        *ITEM_KIND_KILLSWORD, *ITEM_KIND_KROOLCROWN, *ITEM_KIND_LINKARROW, *ITEM_KIND_LINKBOMB, *ITEM_KIND_MAGICBALL, *ITEM_KIND_MAGICPOT, *ITEM_KIND_MASTERBALL, *ITEM_KIND_MAXIMTOMATO, *ITEM_KIND_MECHAKOOPA, *ITEM_KIND_METALBLADE, *ITEM_KIND_METALBLOCK,
        *ITEM_KIND_MURABITOFRUIT, *ITEM_KIND_MUSHROOM, *ITEM_KIND_PACMANAPPLE, *ITEM_KIND_PACMANBELL, *ITEM_KIND_PACMANBOSS, *ITEM_KIND_PACMANCHERRY, *ITEM_KIND_PACMANKEY, *ITEM_KIND_PACMANMELON, *ITEM_KIND_PACMANORANGE, *ITEM_KIND_PACMANSTRAWBERRY,
        *ITEM_KIND_PASARAN, *ITEM_KIND_PELLET, *ITEM_KIND_PITFALL, *ITEM_KIND_POKEBALL, *ITEM_KIND_POWDERBOX, *ITEM_KIND_RAYGUN, *ITEM_KIND_REVENGESHOOTER, *ITEM_KIND_RICHTERHOLYWATER, *ITEM_KIND_ROBOTGYRO, *ITEM_KIND_ROCKETBELT, *ITEM_KIND_SCREW,
        *ITEM_KIND_SENSORBOMB, *ITEM_KIND_SIMONHOLYWATER, *ITEM_KIND_SMARTBOMB, *ITEM_KIND_SMOKESCREEN, *ITEM_KIND_SNAKEGRENADE, *ITEM_KIND_SPECIALFLAG, *ITEM_KIND_STAFF, *ITEM_KIND_STARROD, *ITEM_KIND_STEELDIVER, *ITEM_KIND_SUPERLEAF, *ITEM_KIND_SUPERSCOPE,
        *ITEM_KIND_TEAMHEALFIELD, *ITEM_KIND_TOONLINKBOMB, *ITEM_KIND_UNIRA, *ITEM_KIND_USAGIHAT, *ITEM_KIND_WALKMUSH, *ITEM_KIND_WARPSTAR, *ITEM_KIND_WOOD, *ITEM_KIND_YOUNGLINKBOMB
    ].contains(&kind);
    if ItemModule::is_have_item(boma, 0) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
    }
    if motion_kind == hash40("attack_dash") && frame >= 12.0 {
        if ItemModule::is_have_item(boma, 0) {
            MotionModule::change_motion(boma, Hash40::new("attack_dash_item_light_throw"), 0.0, 1.0, false, 0.0, false, false);
            AttackModule::clear_all(boma);
        }
    }
    if motion_kind == hash40("attack_dash_item_light_throw") {
        if situation_kind != *SITUATION_KIND_AIR && stick_y <= -0.6875 {
            if GroundModule::is_passable_ground(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
        if !SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] {
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SMASH) {
                SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = true;
            }
        }
        if SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] && SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] {
            SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
            SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
            if SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] == 0 {
                SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 1;
                ControlModule::reset_trigger(boma);
                MotionModule::change_motion(boma, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("attack_s4_s3"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    if [hash40("special_n_start"), hash40("special_air_n_start")].contains(&motion_kind) {
        if frame < 11.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                GRENADE_HOLD[entry_id] = false;
            }
            else {
                GRENADE_HOLD[entry_id] = true;
            }
        }
    }
    if status_kind == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WAIT {
        if !ItemModule::is_have_item(boma, 0) {
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(boma, ItemSize{_address: *ITEM_SIZE_HEAVY as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
            ItemModule::pickup_item(boma, ItemSize{_address: *ITEM_SIZE_LIGHT as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        if ItemModule::is_have_item(boma, 0) {
            if heavy_item {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
            }
            else if light_item {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
            }
            else if kind == *ITEM_KIND_GENESIS {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GENESIS_GET, true);
            }
            else if kind == *ITEM_KIND_GRASS {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_GRASS_PULL, true);
            }
            else if kind == *ITEM_KIND_ASSIST {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST, true);
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        snake_start_initialization,
        snake_reset_initialization,
        snake_opff
    );
}