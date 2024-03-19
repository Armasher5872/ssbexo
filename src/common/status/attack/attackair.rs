/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Attack Air Main Common, used for continual platform drops and ECB Shifts
#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.attack_air_common_strans().get_bool() {
        /* START OF NEW ADDITIONS */
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
        let frame = fighter.global_table[CURRENT_FRAME].get_f32();
        let motion_kind = MotionModule::motion_kind(boma);
        let lr = PostureModule::lr(boma);
        let stick_x = fighter.global_table[STICK_X].get_f32()*lr;
        let mut pos = Vector3f {x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)}; // get current pos
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_PASS {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) != true {
                GroundModule::set_passable_check(boma, true);
            }
            if frame <= 1.0 {
                pos.y += 4.5;
                PostureModule::set_pos(boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) 
        && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
        }
        //Captain Falcon Voice Exclamation
        if fighter_kind == *FIGHTER_KIND_CAPTAIN
        && motion_kind == hash40("attack_air_f")
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && frame == 14.0
        && LAST_ATTACK_HITBOX_ID == 0 {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_special_h03"));
            macros::PLAY_SE(fighter, Hash40::new("vc_captain_appeal03"));
        }
        //Sheik Dair Bounce
        if fighter_kind == *FIGHTER_KIND_SHEIK 
        && motion_kind == hash40("attack_air_lw")
        && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
        && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE) {
            let get_sum_speed_x = PostureModule::lr(boma)*KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            fighter.clear_lua_stack();
            lua_args!(fighter, get_sum_speed_x, 2.35, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_animcmd::SET_SPEED_EX(fighter.lua_state_agent);
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        }
        //Angleable Metaknight Dair
        if fighter_kind == *FIGHTER_KIND_METAKNIGHT 
        && motion_kind == hash40("attack_air_lw") 
        && frame < 7.0 {
            if stick_x > 0.5 {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_air_lw_diagonal_r"), -1.0, 1.0, 0.0, false, false);
            }
            if stick_x < -0.5 {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_air_lw_diagonal_l"), -1.0, 1.0, 0.0, false, false);
            }
        }
        /* END OF NEW ADDITIONS */
        if !CancelModule::is_enable_cancel(boma) {
            if MotionModule::is_end(boma) {
                WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            return false.into();
        }
        else {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
                if !MotionModule::is_end(boma) {
                    return false.into();
                }
                WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    true.into()
}

//Status End Attack Air, clears flags
#[skyline::hook(replace = L2CFighterCommon_status_end_AttackAir)]
unsafe fn status_end_attackair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    if fighter_kind == *FIGHTER_KIND_GANON {
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);	
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attackair_main_common,
            status_end_attackair
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}