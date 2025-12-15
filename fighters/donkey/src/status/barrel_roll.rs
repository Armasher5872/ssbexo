use super::*;

unsafe extern "C" fn donkey_barrel_roll_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_barrel_roll_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let owner_situation_kind = StatusModule::situation_kind(owner_boma);
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("speed_min"));
    let brake_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("brake_x"));
    let gravity = WorkModule::get_param_float(weapon.module_accessor, hash40("param_barrel"), hash40("gravity"));
    let angle = 25.0f32;
    let speed_x = angle.to_radians().sin()*speed*owner_lr;
    let speed_y = angle.to_radians().cos()*speed;
    WorkModule::set_int(weapon.module_accessor, -1, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    if owner_situation_kind == *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*2.5, 0.0);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, (-brake_x)*owner_lr, -gravity);
        sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 4.0, 4.0);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(20.0*owner_lr), y: owner_pos_y, z: owner_pos_z});
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*2.5, speed_y*0.5);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, (-brake_x)*owner_lr, -gravity);
        sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -4.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 4.0, 4.0);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(12.0*owner_lr), y: owner_pos_y+6.0, z: owner_pos_z});
    }
    ReflectorModule::set_status_all(weapon.module_accessor, ShieldStatus(*SHIELD_STATUS_NORMAL_GLOBAL), 0);
    ReflectorModule::set_no_team(weapon.module_accessor, true);
    0.into()
}

unsafe extern "C" fn donkey_barrel_roll_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ModelModule::set_joint_translate(weapon.module_accessor, Hash40::new("rotx"), &Vector3f{x: 0.0, y: 3.5, z: 0.0}, false, false);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("roll"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_roll_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_roll_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let mut joint_rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_rotation(weapon.module_accessor, Hash40::new("rotx"), &mut joint_rot, false);
    let owner_boma = get_owner_boma(weapon);
    let abs_horizontal_speed = speed_x.abs();
    let rot_speed_rad = abs_horizontal_speed/7.5;
    let rot_speed_deg = rot_speed_rad.to_degrees();
    let model_rot = Vector3f{x: joint_rot.x+rot_speed_deg, y: 0.0, z: 0.0};
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("rotx"), &model_rot, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    WorkModule::on_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) && frame > 1.0 {
        if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLAG_TOUCHED_GROUND) {
            PLAY_SE(weapon, Hash40::new("se_item_barrel_landing"));
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLAG_TOUCHED_GROUND);
        }
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2f89bbb63a));
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
        ModelModule::set_joint_translate(weapon.module_accessor, Hash40::new("rotx"), &Vector3f{x: 0.0, y: 3.5, z: 0.0}, false, false);
    }
    if !GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLAG_TOUCHED_GROUND);
    }
    if should_remove_barrel(weapon) {
        remove_barrel(weapon);
    }
    if speed_x > -0.01 && speed_x < 0.01 {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE.into(), false.into());
    }
    if life <= 40 {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK.into(), false.into());
    }
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("roll"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn donkey_barrel_roll_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn donkey_barrel_roll_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_barrel_roll_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_ROLL, donkey_barrel_roll_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_ROLL, donkey_barrel_roll_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_ROLL, donkey_barrel_roll_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_ROLL, donkey_barrel_roll_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_ROLL, donkey_barrel_roll_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_ROLL, donkey_barrel_roll_exit_status)
    .install()
    ;
}