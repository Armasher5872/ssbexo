use super::*;

unsafe extern "C" fn snake_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(module_accessor);
    let motion_kind = MotionModule::motion_kind(module_accessor);
    let frame = MotionModule::frame(module_accessor);
    let stick_y = ControlModule::get_stick_y(module_accessor);
    if ItemModule::is_have_item(module_accessor, 0) {
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
    }
    if motion_kind == hash40("attack_dash")
    && frame >= 12.0 {
        if ItemModule::is_have_item(module_accessor, 0) {
            MotionModule::change_motion(module_accessor, Hash40::new("attack_dash_item_light_throw"), 0.0, 1.0, false, 0.0, false, false);
            AttackModule::clear_all(module_accessor);
        }
    }
    if motion_kind == hash40("attack_dash_item_light_throw") {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
        && stick_y <= -0.6875 {
            if GroundModule::is_passable_ground(module_accessor) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
        if SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] == false {
            if ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_SMASH) {
                SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = true;
            }
        }
        if SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id]
        && SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] {
            SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
            SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
            if SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] == 0 {
                SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 1;
                ControlModule::reset_trigger(module_accessor);
                MotionModule::change_motion(module_accessor, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(module_accessor, Hash40::new("attack_s4_s3"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    if [hash40("special_n_start"), hash40("special_air_n_start")].contains(&motion_kind) {
        if frame < 11.0 {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                GRENADE_HOLD[entry_id] = false;
            }
            else {
                GRENADE_HOLD[entry_id] = true;
            }
        }
    }
    if status_kind == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WAIT {
        // if there is a heavy item to pick up nearby, grab it
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_HEAVY as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if there is a light item nearby, grab it
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_LIGHT as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if you have any items at all, transition into pickup
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let heavy_item = [*ITEM_KIND_BARREL, *ITEM_KIND_BOX, *ITEM_KIND_CARRIERBOX, *ITEM_KIND_KUSUDAMA, *ITEM_KIND_SNAKECBOX, *ITEM_KIND_WARIOBIKE].contains(&fighter_kind);
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
        ].contains(&fighter_kind);
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            if heavy_item == true {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
            }
            else if light_item == true {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
            }
            else if fighter_kind == *ITEM_KIND_GENESIS {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GENESIS_GET, true);
            }
            else if fighter_kind == *ITEM_KIND_GRASS {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_GRASS_PULL, true);
            }
            else if fighter_kind == *ITEM_KIND_ASSIST {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST, true);
            }
        }
    }
}

pub fn install() {
    Agent::new("snake")
    .on_line(Main, snake_frame)
    .install()
    ;
}