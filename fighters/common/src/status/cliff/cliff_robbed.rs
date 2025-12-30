use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_CliffRobbed)]
unsafe extern "C" fn status_pre_cliffrobbed(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_FALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_DAMAGE as u32, 0, 0);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_CliffRobbed)]
unsafe extern "C" fn status_cliffrobbed(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let cliff_release_disable_wall_jump_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_release_disable_wall_jump_frame"));
    let speed_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
    let speed_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
    let mut motion_kind = Hash40::new("damage_air_2");
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_CLIFF_ROBBED);
    if fighter_kind == *FIGHTER_KIND_KOOPAG {
        motion_kind = Hash40::new("fall");
    }
    MotionModule::change_motion(fighter.module_accessor, motion_kind, 0.0, 1.0, false, 0.0, false, false);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x*-lr);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.09);
    WorkModule::set_int(fighter.module_accessor, cliff_release_disable_wall_jump_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_WALL_JUMP_FRAME);
    ShakeModule::req(fighter.module_accessor, Hash40::new("damage_air"), 5, false, &Vector2f{x: 0.0, y: 1.0}, 1.0, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CliffRobbed_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_CliffRobbed_Main)]
unsafe extern "C" fn status_cliffrobbed_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_CliffRobbed)]
unsafe extern "C" fn status_end_cliffrobbed(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_WALL_JUMP_FRAME);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_cliffrobbed,
            status_cliffrobbed,
            status_cliffrobbed_main,
            status_end_cliffrobbed
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}