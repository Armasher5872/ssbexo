use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_CliffRobbed)]
unsafe extern "C" fn status_pre_cliffrobbed(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_DAMAGE as u32, 0, 0);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_CliffRobbed)]
unsafe extern "C" fn status_cliffrobbed(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let cliff_release_disable_wall_jump_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("cliff_release_disable_wall_jump_frame"));
    let cliff_robbed_no_control_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("cliff_robbed_no_control_frame"));
    let speed_x = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
    let speed_y = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
    let mut motion_kind = Hash40::new("damage_air_2");
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_CLIFF_ROBBED);
    if fighter_kind == *FIGHTER_KIND_KOOPAG {
        motion_kind = Hash40::new("fall");
    }
    MotionModule::change_motion(boma, motion_kind, 0.0, MotionModule::end_frame_from_hash(boma, motion_kind)/(cliff_robbed_no_control_frame as f32), false, 0.0, false, false);
    KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x*-lr);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.15);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0075);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.32);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.09);
    WorkModule::set_int(boma, cliff_release_disable_wall_jump_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_WALL_JUMP_FRAME);
    ShakeModule::req(boma, Hash40::new("damage_air"), 5, false, &Vector2f{x: 0.0, y: 1.0}, 1.0, 0.0, false, false);
    if !StopModule::is_stop(boma) {
        fighter.sub_cliff_robbed_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_cliff_robbed_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CliffRobbed_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_end_CliffRobbed)]
unsafe extern "C" fn status_end_cliffrobbed(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
    WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_WALL_JUMP_FRAME);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_cliffrobbed,
            status_cliffrobbed,
            status_end_cliffrobbed
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}