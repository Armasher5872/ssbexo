use {
    crate::functions::{
        GRENADE_HOLD,
        SNAKE_INT_ATTACK_S4_COMBO_COUNT,
        SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE,
        SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
    },
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_SNAKE )]
fn snake_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
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
            let character_item = [*ITEM_KIND_DIDDYPEANUTS, *ITEM_KIND_KROOLCROWN, *ITEM_KIND_LINKARROW, *ITEM_KIND_LINKBOMB, *ITEM_KIND_MECHAKOOPA, *ITEM_KIND_METALBLADE, *ITEM_KIND_PACMANAPPLE, *ITEM_KIND_PACMANBELL, *ITEM_KIND_PACMANBOSS, *ITEM_KIND_PACMANCHERRY, *ITEM_KIND_PACMANKEY, *ITEM_KIND_PACMANMELON, *ITEM_KIND_PACMANORANGE, *ITEM_KIND_PACMANSTRAWBERRY, *ITEM_KIND_RICHTERHOLYWATER, *ITEM_KIND_ROBOTGYRO, *ITEM_KIND_SIMONHOLYWATER, *ITEM_KIND_SNAKEGRENADE, *ITEM_KIND_TOONLINKBOMB, *ITEM_KIND_YOUNGLINKBOMB].contains(&fighter_kind);
            if ItemModule::is_have_item(fighter.module_accessor, 0) {
                if heavy_item == true {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
                }
                else if fighter_kind == *ITEM_KIND_GRASS {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_GRASS_PULL, true);
                }
                else if fighter_kind == *ITEM_KIND_ASSIST {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST, true);
                }
                else {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
                }
                if character_item == true {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
                }
            }
        }
    }
}

#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_side_smash_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_side_smash_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
    SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
    SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 0;
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        snake_side_smash_status_main,
        snake_side_smash_status_end
    );
    install_agent_frames!(
        snake_frame
    );
}