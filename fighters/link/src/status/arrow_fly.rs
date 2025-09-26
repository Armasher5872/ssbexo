use super::*;

unsafe extern "C" fn link_bowarrow_fly_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    WorkModule::set_float(weapon.module_accessor, WorkModule::get_float(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_DEGREE), *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_SHOT_ANGLE);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let lr = PostureModule::lr(weapon.module_accessor);
    let arrow_type = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    let power_min = match arrow_type {
        _ if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW => {
            1.2
        },
        _ if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW => {
            20.0
        },
        _ => {
            WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("power_min"))
        }
    };
    let power_max = match arrow_type {
        _ if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW => {
            4.4
        },
        _ if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW => {
            20.0
        },
        _ => {
            WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("power_max"))
        }
    };
    let speed_min = match arrow_type {
        _ if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW => {
            3.6
        },
        _ if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW => {
            5.0
        },
        _ if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW => {
            2.4
        },
        _ if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW => {
            300.0
        },
        _ => {
            WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("speed_min"))
        }
    };
    let speed_max = match arrow_type {
        _ if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW => {
            4.5
        },
        _ if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW => {
            6.25
        },
        _ if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW => {
            3.0
        },
        _ if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW => {
            300.0
        },
        _ => {
            WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("speed_max"))
        }
    };
    let accel_y = match arrow_type {
        _ if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW => {
            WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("accel_y"))/2.0
        },
        _ if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW => {
            0.0
        },
        _ => {
            WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("accel_y"))
        }
    };
    let charge = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let shot_angle = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_SHOT_ANGLE);
    let power = power_max-power_min;
    let power_charge = power*charge;
    let lerp_speed = weapon.lerp(speed_min.into(), speed_max.into(), charge.into());
    let speed_x = ((shot_angle+90.0).to_radians().sin()*lerp_speed.get_f32()*lr)*0.95;
    let speed_y = (shot_angle-90.0).to_radians().cos()*lerp_speed.get_f32();
    let reduction = if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW {shot_angle.abs()/720.0} else {shot_angle.abs()/450.0};
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_DOUBLE) {
        WorkModule::set_int(weapon.module_accessor, ((power_min+power_charge)*0.75) as i32, *WN_LINK_BOWARROW_STATUS_FLY_WORK_INT_ATTACK_POWER);
    }
    else {
        WorkModule::set_int(weapon.module_accessor, (power_min+power_charge) as i32, *WN_LINK_BOWARROW_STATUS_FLY_WORK_INT_ATTACK_POWER);
    }
    if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW {
        if WorkModule::is_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE) {
            EFFECT_FOLLOW(weapon, Hash40::new("link_light_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, -shot_angle, 0, 0, 1.0, true);
            EFFECT(weapon, Hash40::new("link_light_arrow"), Hash40::new("top"), 0, 0, 0, -shot_angle, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            ModelModule::set_scale(weapon.module_accessor, 0.001);
        }
    }
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(10.0*lr), y: owner_pos_y+11.0, z: owner_pos_z});
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x-reduction, speed_y);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -accel_y-reduction);
    0.into()
}

unsafe extern "C" fn link_bowarrow_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let attack_power = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_STATUS_FLY_WORK_INT_ATTACK_POWER);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    AttackModule::set_power(weapon.module_accessor, 0, attack_power as f32, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        link_bowarrow_fly_sub_status(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(link_bowarrow_fly_sub_status as *const () as _));
    weapon.fastshift(L2CValue::Ptr(link_bowarrow_fly_main_loop as *const () as _))
}

unsafe extern "C" fn link_bowarrow_fly_sub_status(weapon: &mut L2CWeaponCommon, bool_check: L2CValue) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let rotate_z = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_STATUS_FLY_WORK_FLOAT_ROTATE_Z);
    let rotate_speed = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_STATUS_FLY_WORK_FLOAT_ROTATE_SPEED);
    let mut z = rotate_z+rotate_speed;
    if bool_check.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if life <= 0 {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2b8a2bd943));
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            return 0.into();
        }
    }
    if 360.0 < z {
        z = z-360.0;
    }
    WorkModule::set_float(weapon.module_accessor, z, *WN_LINK_BOWARROW_STATUS_FLY_WORK_FLOAT_ROTATE_Z);
    0.into()
}

unsafe extern "C" fn link_bowarrow_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let arrow_type = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    let ret = GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32);
    if ret {
        if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
            weapon.change_status(WN_LINK_BOWARROW_STATUS_KIND_STICK.into(), false.into());
        }
        else {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2b8a2bd943));
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        }
    }
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR) && WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        WorkModule::on_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
        let item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        let team_no = TeamModule::team_no(weapon.module_accessor) as i32;
        let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor) as u32;
        TeamModule::set_team(item_boma, team_no, true);
        TeamModule::set_team_owner_id(item_boma, team_owner_id);
    }
    ret.into()
}

unsafe extern "C" fn link_bowarrow_fly_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let status_kind_next = StatusModule::status_kind_next(weapon.module_accessor);
    let item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
    if ![*WN_LINK_BOWARROW_STATUS_KIND_STICK, *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK].contains(&status_kind_next) {
        if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) && sv_battle_object::is_active(item_id) {
            LinkModule::remove_model_constraint(item_boma, true);
            StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
        }
    }
    WorkModule::off_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE);
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}

unsafe extern "C" fn link_bowarrow_fly_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    WorkModule::off_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE);
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}

pub fn install() {
    Agent::new("link_bowarrow")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *WN_LINK_BOWARROW_STATUS_KIND_FLY, link_bowarrow_fly_init_status)
    .status(Main, *WN_LINK_BOWARROW_STATUS_KIND_FLY, link_bowarrow_fly_main_status)
    .status(End, *WN_LINK_BOWARROW_STATUS_KIND_FLY, link_bowarrow_fly_end_status)
    .status(Exit, *WN_LINK_BOWARROW_STATUS_KIND_FLY, link_bowarrow_fly_exit_status)
    .install()
    ;
}