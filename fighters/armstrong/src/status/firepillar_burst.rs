use super::*;

unsafe extern "C" fn armstrong_firepillar_burst_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn armstrong_firepillar_burst_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_firepillar"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(owner_boma, weapon.battle_object_id as i32, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_FLAME_PILLAR_ID);
    0.into()
}

unsafe extern "C" fn armstrong_firepillar_burst_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("burst"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(armstrong_firepillar_burst_main_loop as *const () as _))
}

unsafe extern "C" fn armstrong_firepillar_burst_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_X);
    let pos_y = WorkModule::get_float(weapon.module_accessor, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_Y);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if frame < 3.0 {
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(pos_x*owner_lr), y: owner_pos_y+pos_y, z: owner_pos_z});
    }
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn armstrong_firepillar_burst_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    WorkModule::set_int(owner_boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_FLAME_PILLAR_ID);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_INT_EFFECT_ID);
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_X);
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_Y);
    0.into()
}

pub fn install() {
    Agent::new("ganon_firepillar")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .status(Pre, *WEAPON_ARMSTRONG_FIREPILLAR_STATUS_KIND_BURST, armstrong_firepillar_burst_pre_status)
    .status(Init, *WEAPON_ARMSTRONG_FIREPILLAR_STATUS_KIND_BURST, armstrong_firepillar_burst_init_status)
    .status(Main, *WEAPON_ARMSTRONG_FIREPILLAR_STATUS_KIND_BURST, armstrong_firepillar_burst_main_status)
    .status(End, *WEAPON_ARMSTRONG_FIREPILLAR_STATUS_KIND_BURST, armstrong_firepillar_burst_end_status)
    .install()
    ;
}