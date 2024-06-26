use super::*;

const PZENIGAME_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xfeef30; //Squirtle only
const PZENIGAME_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PZENIGAME_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xf95c90; //Shared

//Squirtle Startup Initialization
#[skyline::hook(offset = PZENIGAME_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pzenigame_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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

//Squirtle Reset Initialization
#[skyline::hook(offset = PZENIGAME_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pzenigame_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_PZENIGAME as u32 {
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
    }
}

//Squirtle Once Per Fighter Frame
#[skyline::hook(offset = PZENIGAME_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn pzenigame_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PZENIGAME as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
        let situation_kind = StatusModule::situation_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let kinetic_type = KineticModule::get_kinetic_type(boma);
        let frame = MotionModule::frame(boma);
        let pos_x = PostureModule::pos_x(boma);
        let pos_y = PostureModule::pos_y(boma);
        let lr = PostureModule::lr(boma);
        let point_offset_x1 = lr*(RAIN_DANCE_X1[entry_id]-pos_x);
        let point_offset_x2 = lr*(RAIN_DANCE_X2[entry_id]-pos_x);
        let point_offset_y1 = RAIN_DANCE_Y1[entry_id]-pos_y;
        if [hash40("attack_s4_s"), hash40("attack_s4_hi"), hash40("attack_s4_lw")].contains(&motion_kind)
        && (14.0..17.0).contains(&frame) {
            if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL
        && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL) {
            MotionModule::change_motion(boma, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
        }
        if motion_kind == hash40("special_shield") {
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        };
        if motion_kind == hash40("special_shield") && frame == 50.0 {
            if !RAIN_DANCE_ACTIVE[entry_id] || RAIN_DANCE_FRAME[entry_id] != 600.0 {
                RAIN_DANCE_ACTIVE[entry_id] = true;
                RAIN_DANCE_FRAME[entry_id] = 600.0;
                RAIN_DANCE_X1[entry_id] = pos_x - (18.0 * lr);
                RAIN_DANCE_X2[entry_id] = pos_x + (18.0 * lr);
                RAIN_DANCE_Y1[entry_id] = pos_y;
                RAIN_DANCE_Y2[entry_id] = pos_y + 20.0;
            }
        }
        if (pos_x <= RAIN_DANCE_X2[entry_id] && pos_x >= RAIN_DANCE_X1[entry_id]) && (pos_y >= RAIN_DANCE_Y1[entry_id] && pos_y <= RAIN_DANCE_Y2[entry_id]) && IN_RAIN_DANCE[entry_id] != true {
            IN_RAIN_DANCE[entry_id] = true;
        }
        else {
            IN_RAIN_DANCE[entry_id] = false;
        }
        if RAIN_DANCE_ACTIVE[entry_id] {
            RAIN_DANCE_FRAME[entry_id] -= 1.0;
            if RAIN_DANCE_FRAME[entry_id] <= 600.0 && RAIN_DANCE_FRAME[entry_id] >= 598.0 {
                EffectModule::kill_kind(boma, Hash40::new("sys_ground_shockwave"), false, false);
            }
            else if RAIN_DANCE_FRAME[entry_id] < 598.0 && RAIN_DANCE_FRAME[entry_id] % 30.0 == 0.0 {
                EffectModule::req_follow(boma, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), &Vector3f{x: point_offset_x1, y: point_offset_y1, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.5, false, 0, 0, 0, 0, 0, false, false);
                EffectModule::req_follow(boma, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), &Vector3f{x: point_offset_x2, y: point_offset_y1, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.5, false, 0, 0, 0, 0, 0, false, false);
                EffectModule::req_follow(boma, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), &Vector3f{x: point_offset_x1-(12.0*lr), y: point_offset_y1, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.5, false, 0, 0, 0, 0, 0, false, false);
                EffectModule::req_follow(boma, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), &Vector3f{x: point_offset_x2+(12.0*lr), y: point_offset_y1, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.5, false, 0, 0, 0, 0, 0, false, false);
            }
            if RAIN_DANCE_FRAME[entry_id] <= 0.0 {
                RAIN_DANCE_ACTIVE[entry_id] = false;
            }
        }
        if !RAIN_DANCE_ACTIVE[entry_id] {
            RAIN_DANCE_FRAME[entry_id] = -1.0;
            RAIN_DANCE_X1[entry_id] = 0.0;
            RAIN_DANCE_X2[entry_id] = 0.0;
            RAIN_DANCE_Y1[entry_id] = 0.0;
            RAIN_DANCE_Y2[entry_id] = 0.0;
            EffectModule::kill_kind(boma, Hash40::new("sys_ground_shockwave"), false, false);
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if situation_kind == *SITUATION_KIND_AIR {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
        if status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE {
            if situation_kind == *SITUATION_KIND_AIR {
                if fighter.battle_object.is_cat_flag(Cat1::AirEscape) {
                    WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                    ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                }
            }
        }
        if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
            if MotionModule::frame(boma) >= 10.0 && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    PZENIGAME_WITHDRAW_JUMP[entry_id] = 1;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
                else if situation_kind == *SITUATION_KIND_AIR
                && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    PZENIGAME_WITHDRAW_JUMP[entry_id] = 1;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                }
            }
        }
        //Side Special Related Stuff
        if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) && PZENIGAME_WITHDRAW_JUMP[entry_id] == 1 {
            MotionModule::set_rate(boma, 0.5);
            PZENIGAME_WITHDRAW_JUMP[entry_id] = 0;
        }
        if [*FIGHTER_KINETIC_TYPE_JUMP, *FIGHTER_KINETIC_TYPE_JUMP_ICE, *FIGHTER_KINETIC_TYPE_JUMP_CLIFF, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_CLIFF_VERTICAL, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND].contains(&kinetic_type) && PZENIGAME_WITHDRAW_JUMP[entry_id] == 1 {
            PZENIGAME_WITHDRAW_JUMP[entry_id] = 0;
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&prev_status_kind) && PZENIGAME_WITHDRAW_JUMP[entry_id] != 0 {
                PZENIGAME_WITHDRAW_JUMP[entry_id] = 0;
            }
            if PZENIGAME_WITHDRAW_JUMP[entry_id] == 0 {
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        pzenigame_start_initialization,
        pzenigame_reset_initialization,
        pzenigame_opff
    );
}