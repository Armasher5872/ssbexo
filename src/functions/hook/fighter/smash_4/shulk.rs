use super::*;

const SHULK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x11623f0; //Shulk Only
const SHULK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1162490; //Shulk Only
const SHULK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x1163580; //Shulk Only
const SHULK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x1164200; //Shulk Only
const SHULK_CHECK_VALID_ART_STATUSES_OFFSET: usize = 0x116a3d0; //Shulk Only
const SHULK_CHECK_CAN_ACTIVATE_ART_WHEEL: usize = 0x116d8a0; //Shulk Only

//Shulk Startup Initialization
#[skyline::hook(offset = SHULK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn shulk_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Shulk Reset Initialization
#[skyline::hook(offset = SHULK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn shulk_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Shulk Death Initialization
#[skyline::hook(offset = SHULK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn shulk_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Shulk Once Per Fighter Frame
#[skyline::hook(offset = SHULK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn shulk_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    //Final Zoom Effect Clearing
    if counter > 0 {
        if counter == 20 {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
                set_stage_visibility(boma, 1);
                set_vis_hud(true);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_shulk_final"), -1);
                EffectModule::set_rate(boma, handle as u32, 1.0);
            }
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_black"), false, false);
            macros::CAM_ZOOM_OUT(agent);
        }
        if counter == 10 {
            SlowModule::clear_whole(boma);
        }
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    }
    else {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    }
    original!()(vtable, fighter)
}

//Shulk Check Valid Arts Statuses
#[skyline::hook(offset = SHULK_CHECK_VALID_ART_STATUSES_OFFSET)]
unsafe extern "C" fn shulk_check_valid_arts_statuses(fighter: *mut Fighter) -> u64 {
    let boma = (*fighter).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    u64::from([
        *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_JUMP, 
        *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_FLY, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_OTTOTTO_WAIT, *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP, *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL, *FIGHTER_STATUS_KIND_ITEM_SCREW_FALL, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP, 
        *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER_KEEP, *FIGHTER_STATUS_KIND_KILLER_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, 
        *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_PASS, *FIGHTER_STATUS_KIND_PLATE_WAIT, *FIGHTER_STATUS_KIND_LADDER, *FIGHTER_STATUS_KIND_LADDER_ATTACK, *FIGHTER_STATUS_KIND_LADDER_END, *FIGHTER_STATUS_KIND_APPEAL
    ].contains(&status_kind))
}

//Shulk Check Can Activate Art Wheel
#[skyline::hook(offset = SHULK_CHECK_CAN_ACTIVATE_ART_WHEEL)]
unsafe extern "C" fn shulk_check_can_activate_art_wheel(fighter: *mut Fighter) -> u64 {
    let boma = (*fighter).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    u64::from([
        *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_JUMP_SQUAT,
        *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_FLY, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_OTTOTTO_WAIT, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_SHULK_SPECIAL_N
    ].contains(&status_kind))
}

pub fn install() {
    skyline::install_hooks!(
        shulk_start_initialization,
        shulk_reset_initialization,
        shulk_death_initialization,
        shulk_opff,
        shulk_check_valid_arts_statuses,
        shulk_check_can_activate_art_wheel
    );
}