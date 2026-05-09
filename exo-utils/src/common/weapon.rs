use super::*;

pub unsafe extern "C" fn should_remove_projectile(weapon: &mut L2CWeaponCommon) -> bool {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    || life <= 0 {
        return true;
    }
    return false;
}

pub fn weapon_initialise_module(weapon_id: i32, module: ModuleInitModules) {
    let module_init_offset = match module {
        ModuleInitModules::KineticModule => 0x33b83f0,
        ModuleInitModules::ArticleModule => 0x33b9200,
        ModuleInitModules::AttackModule => 0x33b8540,
        ModuleInitModules::ControlModule => 0x33b9000,
        ModuleInitModules::EffectModule => 0x33b8730,
        ModuleInitModules::GroundModule => 0x33b8850,
        ModuleInitModules::MotionModule => 0x33b7d50,
        ModuleInitModules::ReflectModule => 0x33b89d0,
        ModuleInitModules::SearchModule => 0x33b8a80,
        ModuleInitModules::SoundModule => 0x33b8c60,
        ModuleInitModules::VisibilityModule => 0x33b7ec0,
        ModuleInitModules::ColorBlendModule => 0x33b7f60,
        ModuleInitModules::ShakeModule => 0x33b7ff0,
        ModuleInitModules::AreaModule => 0x33b8f50,
        ModuleInitModules::SlopeModule => 0x33b9170,
        ModuleInitModules::ReflectorModule => 0x33b9830,
        ModuleInitModules::SlowModule => 0x33b9350,
        ModuleInitModules::MotionAnimcmdModule => 0x33b81d0,
        ModuleInitModules::TurnModule => 0x33b9940,
        ModuleInitModules::LuaModule => 0x33b8e40,
        _ => 0x0,
    };
    if module_init_offset == 0 {
        return;
    }
    let offset = 0x5189818+(0xe8*(weapon_id as usize))+(module as usize*0x8);
    let module_init = unsafe {skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64+module_init_offset};
    let _ = skyline::patching::Patch::in_text(offset).data(module_init);
}