use super::*;

const IKE_SLASH_VTABLE_INITIALIZATION_EVENT_OFFSET: usize = 0x3425b40;

//Ike Ragnellian Slash Initialization Event Offset
#[skyline::hook(offset = IKE_SLASH_VTABLE_INITIALIZATION_EVENT_OFFSET)]
unsafe extern "C" fn ike_slash_initialization_event(vtable: u64, weapon: *mut smash::app::Weapon, param_3: u64) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
    let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_IKE {
        let shield_data = ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, Hash40::new("rot"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
        let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
        let resource = &mut ShieldGroupResource2::new(shield_datas, 1, 4.0, 4.0, 100.0, 20.0, false, 0);
        add_reflector_group(boma, resource, *WEAPON_IKE_SLASH_REFLECTOR_KIND_REFLECTOR);
        ReflectorModule::set_hop(boma, true, 45.0, *WEAPON_IKE_SLASH_REFLECTOR_KIND_REFLECTOR);
    }
    call_original!(vtable, weapon, param_3)
}

pub fn install() {
    skyline::install_hook!(ike_slash_initialization_event);
}