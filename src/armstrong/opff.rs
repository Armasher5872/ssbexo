use super::*;

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
fn armstrong_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        //Damage Update
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
        || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
            USE_DROPKICK[entry_id] = true;
        }
        //Instadrop
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) == true {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 75, 100, 0, 50, 7.0, 0.0, 2.0, 0.0, Some(0.0), Some(10.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
        if [*FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
            AttackModule::clear_all(boma);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_mach_stomp"), false, false);
        }
        //Neutral Special
        if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
            if motion_kind == hash40("special_air_n") {
                WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
            }
            else {
                if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE) {
                    MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_lw_end"), 1.0, 1.0, 0.0, false, false);
                    AttackModule::clear_all(boma);
                }
            }
        }
        else {
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        }
        //Down Special End
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            }
        }
        if status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END {
            AttackModule::clear_all(boma);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        armstrong_frame
    );
}