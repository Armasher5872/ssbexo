use super::*;

const IKE_SLASH_VTABLE_INITIALIZATION_EVENT_OFFSET: usize = 0x3425ae0;

//Ike Ragnellian Slash Initialization Event Offset
#[skyline::hook(offset = IKE_SLASH_VTABLE_INITIALIZATION_EVENT_OFFSET)]
unsafe extern "C" fn ike_slash_initialization_event(vtable: u64, weapon: *mut smash::app::Weapon, param_3: u64) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = *(param_3 as *mut u32).add(0x2c/4);
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
    let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_IKE {
        let shield_data = ShieldData::new(0.0, 11000.0, 0.0, 0.0, -7000.0, 0.0, 4000.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_JUST_SHIELD_REFLECTOR as u8);
        let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
        let resource = &mut ShieldGroupResource2::new(shield_datas, 1, 1.5, 1.5, 50.0, 1.0, false, 0);
        let ptr = get_module_vtable_func(boma, 0x108, 0x60);
        let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource2, i32) = std::mem::transmute(ptr);
        let reflector_module = *(boma as *mut *mut u64).add(0x108/8);
        set_shield_group2(reflector_module, resource, *WEAPON_IKE_SLASH_REFLECTOR_KIND_REFLECTOR);
        ReflectorModule::set_status(boma, *WEAPON_IKE_SLASH_REFLECTOR_KIND_REFLECTOR, smash::app::ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    }
    call_original!(vtable, weapon, param_3)
}

//Ike Ragnellian Slash Reflector Clean Event Offset
unsafe extern "C" fn ike_slash_reflector_clean_event(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    ReflectorModule::clean(boma);
}

//Ike Ragnellian Slash On Reflection Event Offset
unsafe extern "C" fn ike_slash_on_reflection_event(_vtable: u64, weapon: *mut smash::app::Weapon, log: *mut ShieldAttackCollisionEvent) {
    let boma = (*weapon).battle_object.module_accessor;
    let attacker_article_id = (*(*log).collision_log).opponent_object_id;
    let attacker_article_category = (*(*log).collision_log).opponent_object_category as i32;
    let attacker_article_power = (*log).real_power;
    let attacker_article_battle_object = get_battle_object_from_id(attacker_article_id);
    let attacker_article_boma = (*attacker_article_battle_object).module_accessor;
    let attacker_article_agent = get_weapon_common_from_accessor(&mut *attacker_article_boma);
    let attacker_article_speed = KineticModule::get_sum_speed_x(attacker_article_boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if [*BATTLE_OBJECT_CATEGORY_WEAPON, *BATTLE_OBJECT_CATEGORY_ITEM].contains(&attacker_article_category) {
        if attacker_article_category == *BATTLE_OBJECT_CATEGORY_ITEM {
            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
            if smash::app::sv_battle_object::is_active(attacker_article_id) {
                smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, attacker_article_id);
            }
        }
        notify_event_msc_cmd!(attacker_article_agent, Hash40::new_raw(0x199c462b5d));
        WorkModule::set_float(boma, attacker_article_speed, *WEAPON_IKE_SLASH_INSTANCE_WORK_ID_FLOAT_ABSORB_SPEED);
        WorkModule::set_float(boma, attacker_article_power, *WEAPON_IKE_SLASH_INSTANCE_WORK_ID_FLOAT_ABSORB_POWER);
        WorkModule::on_flag(boma, *WEAPON_IKE_SLASH_INSTANCE_WORK_ID_FLAG_ABSORBED_WEAPON);
    }
}

pub fn install() {
    //Fuck it ball type code (Patches the initialization of Bowser Jr's Cannonball modules to instead use Palutena's Reflection Board Module Initialization so that the former can call to ReflectorModule functions correctly)
    let initialize_reflectormodule = unsafe {skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64+0x33b9aa0};
    let _ = skyline::patching::Patch::in_text(0x519bb68).data(initialize_reflectormodule);
    let _ = skyline::patching::Patch::in_text(0x51d9348).data(ike_slash_reflector_clean_event as u64);
    let _ = skyline::patching::Patch::in_text(0x51d9468).data(ike_slash_on_reflection_event as u64);
    skyline::install_hook!(ike_slash_initialization_event);
}