use super::*;

const RICHTER_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x1194120; //Shared
const RICHTER_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1194130; //Shared
const RICHTER_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11944e0; //Shared
pub static mut RICHTER_HOLYWATER: usize = 0x758e00;

unsafe extern "C" fn richter_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP);
    }
    0.into()
}

//Richter Startup Initialization
#[skyline::hook(offset = RICHTER_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn richter_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_RICHTER as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(richter_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Richter Reset Initialization
#[skyline::hook(offset = RICHTER_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn richter_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_RICHTER as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Richter Death Initialization
#[skyline::hook(offset = RICHTER_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn richter_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_RICHTER as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Credited to WuBoyTH
#[skyline::hook(replace = RICHTER_HOLYWATER)]
unsafe extern "C" fn richter_holywater_born_some_status(item: &mut L2CAgent) -> L2CValue {
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_BODY);
    disable_area(item.lua_state_agent, *ITEM_AREA_KIND_BODY);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    HitModule::set_whole(item.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
    WorkModule::off_flag(item.module_accessor, *ITEM_INSTANCE_WORK_FLAG_AUTO_PLAY_LOST_EFFECT);
    KineticModule::clear_speed_all(item.module_accessor);
    let kind = richter_holywater_owner_kind(item).get_i32();
    let gravity_accel = fire_pillar_gravity_accel(FighterKind(kind));
    item.clear_lua_stack();
    lua_args!(item, -gravity_accel);
    kinetic_energy_gravity_set_accel(item.lua_state_agent, -gravity_accel);
    let gravity_accel_max = fire_pillar_gravity_accel_max(FighterKind(kind));
    item.clear_lua_stack();
    lua_args!(item, gravity_accel_max);
    kinetic_energy_gravity_set_limit_speed(item.lua_state_agent, gravity_accel_max);
    item.clear_lua_stack();
    lua_args!(item, 0, 0, 0);
    kinetic_energy_control_rot_set_rotation(item.lua_state_agent, &Vector3f::zero());
    item.clear_lua_stack();
    lua_args!(item, 0, 0, 0);
    kinetic_energy_rot_set_rotation(item.lua_state_agent, &Vector3f::zero());
    if !GroundModule::is_touch(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        KineticModule::add_speed(item.module_accessor, &Vector3f{x: 0.0, y: fire_pillar_speed_y(FighterKind(kind)), z: 0.0});
    }
    if !GroundModule::is_touch(item.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
        let speed_x = 0.4;
        let lr = PostureModule::lr(item.module_accessor);
        let speed = speed_x*lr;
        item.clear_lua_stack();
        lua_args!(item, 0, 0);
        kinetic_energy_control_set_accel(item.lua_state_agent, &Vector2f::zero());
        item.clear_lua_stack();
        lua_args!(item, 0, 0);
        kinetic_energy_control_set_brake(item.lua_state_agent, &Vector2f::zero());
        item.clear_lua_stack();
        lua_args!(item, speed, 0);
        kinetic_energy_control_set_stable_speed(item.lua_state_agent, &Vector2f{x: speed, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, speed, 0);
        kinetic_energy_control_set_limit_speed(item.lua_state_agent, &Vector2f{x: speed, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, speed, 0);
        kinetic_energy_control_set_speed(item.lua_state_agent, &Vector2f{x: speed, y: 0.0});
        item.clear_lua_stack();
        kinetic_energy_control_enable(item.lua_state_agent);
    }
    PostureModule::set_rot(item.module_accessor, &Vector3f::zero(), 0);
    0.into()
}

unsafe extern "C" fn richter_holywater_owner_kind(_item: &mut L2CAgent) -> L2CValue {
    0x44.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            RICHTER_HOLYWATER += (*info.module.ModuleObject).module_base as usize;
            skyline::install_hook!(richter_holywater_born_some_status);
        }
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
	skyline::install_hooks!(
        richter_start_initialization,
        richter_reset_initialization,
        richter_death_initialization
    );
}