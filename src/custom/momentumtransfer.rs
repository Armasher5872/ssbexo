#![allow(
    illegal_floating_point_literal_pattern,
    unused_macros,
    unused_mut,
    unused_variables
)]
use {
    crate::functions::{
        CURRENT_MOMENTUM,
        CURRENT_MOMENTUM_SPECIALS,
        FIGHTER_KIND,
        GROUND_VEL,
        JUMP_SQUAT_FRAME,
        JUMPSQUAT_VELOCITY,
        JUMP_SPEED_MAX_MUL,
        JUMP_SPEED_RATIO,
        RAR_LENIENCY,
        BomaExt,
        FrameInfo
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lua2cpp::L2CFighterCommon,
        lib::{
            L2CValue,
            lua_const::*,
        }
    },
    smash_script::*,
};

pub unsafe fn calc_melee_momentum(fighter: &mut L2CFighterCommon, aerial_attack: bool, attack_cancel: bool, walking: bool) -> f32 {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let jump_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x"), 0);
    let jump_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_mul"), 0);
    let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
    let ratio = JUMP_SPEED_RATIO;
    let mut jump_speed_max_mul = JUMP_SPEED_MAX_MUL;
    // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
    match jump_speed_max_mul {0.1..=3.0 => {}, _ => { jump_speed_max_mul = 1.0 }}
    let jump_speed_x_max = run_speed_max * ratio * jump_speed_max_mul;
    let mut x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT
    || ([*FIGHTER_KIND_PICKEL, *FIGHTER_KIND_TANTAN].contains(&fighter_kind) && [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT].contains(&status_kind)) {
        x_vel = JUMPSQUAT_VELOCITY;
    }
    // Calculate jump momentum based on the momentum you had on the last frame of jumpsquat
    let mut calc_jump_speed = (x_vel * jump_speed_x_mul) + (jump_speed_x * stick_x);
    //melee jump speed calculation... courtesey of Brawltendo
    let jump_speed_clamped = calc_jump_speed.clamp(-jump_speed_x_max, jump_speed_x_max);
    jump_speed_clamped
}

pub unsafe fn momentum_transfer_helper(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_EFLAME, *FIGHTER_KIND_ELIGHT].contains(&fighter_kind) && [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT].contains(&status_kind)
    || status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
		let mut ratio = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0) / WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
        JUMP_SPEED_RATIO = ratio;
	}
	if [*FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) {
        // You have a limited amount of time to get full RAR momentum from turn brake or run brake, with a 3F leniency
        RAR_LENIENCY = (0.8*(MotionModule::end_frame(boma) - MotionModule::frame(boma)*2.0 + 6.0)/MotionModule::end_frame(boma)).clamp(0.1, 0.8);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        GROUND_VEL = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
	if [*FIGHTER_KIND_PICKEL, *FIGHTER_KIND_TANTAN].contains(&fighter_kind) && [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT].contains(&status_kind) 
    && JUMP_SQUAT_FRAME < WorkModule::get_param_int(boma, hash40("jump_squat_frame"), 0) {
        JUMPSQUAT_VELOCITY = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}
	if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_JUMP {
        let mut new_speed = calc_melee_momentum(fighter, false, false, false);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
        smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        // Set the current momentum to what was just calculated
        CURRENT_MOMENTUM = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_common)]
pub unsafe fn status_attack_air_hook(fighter: &mut L2CFighterCommon, param_1: L2CValue){
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let ratio = JUMP_SPEED_RATIO;
    // get the multiplier for any special mechanics that require additional jump speed max
    let mut jump_speed_max_mul = JUMP_SPEED_MAX_MUL;
    // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
    match jump_speed_max_mul {0.1..=3.0 => {}, _ => { jump_speed_max_mul = 1.0 }}
    let mut jump_speed_x_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0) * ratio * jump_speed_max_mul;
    let new_speed = CURRENT_MOMENTUM.clamp(-jump_speed_x_max, jump_speed_x_max);
    if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_JUMP {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
        smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
    original!()(fighter, param_1)
}

unsafe fn additional_momentum_transfer_moves(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, curr_frame: f32) {
    /*      ADDITIONAL MOVES THAT SHOULD CONSERVE MOMENTUM       */
    if situation_kind == *SITUATION_KIND_AIR && curr_frame <= 1.0 {
        //characters whose specials should conserve momentum
        let should_conserve_special_momentum = fighter_kind == *FIGHTER_KIND_ALL && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind);
		if should_conserve_special_momentum && (KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN)).abs() > 0.0 {
			if *FIGHTER_STATUS_KIND_JUMP == StatusModule::prev_status_kind(boma, 0) {
				let new_speed = CURRENT_MOMENTUM_SPECIALS;
				fighter.clear_lua_stack();
				lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
				smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
				fighter.clear_lua_stack();
			}
        }

    }
}

pub unsafe fn change_kinetic_momentum_related(boma: &mut smash::app::BattleObjectModuleAccessor, kinetic_type: i32) -> Option<i32> { //special move momentum conservation
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let situation_kind = StatusModule::situation_kind(boma);
    let fighter_kind = boma.kind();
    if (fighter_kind == *FIGHTER_KIND_ALL && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind)) && situation_kind == *SITUATION_KIND_AIR && [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&prev_status_kind) {
        return Some(-1);
    }
    None
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    let curr_frame = MotionModule::frame(boma);
    momentum_transfer_helper(fighter, boma, status_kind, situation_kind, fighter_kind);
    additional_momentum_transfer_moves(fighter, boma, status_kind, situation_kind, fighter_kind, curr_frame);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Jump_Main)]
unsafe extern "C" fn status_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    CURRENT_MOMENTUM = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    CURRENT_MOMENTUM_SPECIALS = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_stop_ceil().get_bool() {
        return 1.into();
    } 
    if !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_air_check_superleaf_fall_slowly();

    } 
    else {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
    };
    0.into()
}

fn nro_main(nro: &skyline::nro::NroInfo) {
    if nro.name == "common" {
        skyline::install_hooks!(
            status_jump_main
        );
    }
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                // lua2cpp_common.nro hooks go here
                status_attack_air_hook,
            );
        }
        _ => (),
    }
}

pub unsafe fn momentum_install(fighter: &mut L2CFighterCommon, info: &FrameInfo) {
    let boma = &mut *info.boma;
    run(fighter, boma, info.status_kind, info.situation_kind, info.fighter_kind);
}

pub fn install() {
    skyline::nro::add_hook(nro_main).unwrap();
}