use super::*;

//Checks what alt you are
pub unsafe fn get_player_number(module_accessor:  &mut smash::app::BattleObjectModuleAccessor) -> usize {
    let player_number;
    if smash::app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
        player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
    }
    else if get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    else {
        let mut owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        while get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
            owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        }
        player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    return player_number;
}

//Gets attacker number
pub unsafe fn get_attacker_number(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> usize {
	let attacker_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR) as usize;
	return attacker_number;
}

//Gets the boma
pub unsafe fn get_boma(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
	let boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(entry_id));
	return boma;
}

//Gets Article Boma
pub unsafe fn get_article_boma(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut BattleObjectModuleAccessor {
    let article = ArticleModule::get_article(boma, article_type);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    return sv_battle_object::module_accessor(object_id);
}

//Used for GGST COUNTER!
pub unsafe fn estimate_frame(module_accessor: &mut smash::app::BattleObjectModuleAccessor, frame: f32) -> bool {
    let motion_frame = MotionModule::frame(module_accessor);
	if motion_frame >= frame && motion_frame < frame + 1.0 {
		return true;
	}
	else {
		return false;
	}
}

//Gets the article owner boma
pub unsafe fn get_owner_boma(weapon: &mut L2CAgentBase) -> *mut BattleObjectModuleAccessor {
    return &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
}

//Collision Log
pub struct CollisionLog {
    pub next: *mut CollisionLog,
    pub end: *mut CollisionLog,
    pub location: Vector3f,
    pub padding_0: u32,
    pub padding_1: u32,
    pub opponent_battle_object_id: u32,
    pub padding_2: [u8;7],
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
    pub padding_3: [u8;10]
}

//Shield Data Resource
#[repr(C)]
pub struct ShieldDataResource {
    pub offset: smash2::cpp::simd::Vector3,
    pub offset2: smash2::cpp::simd::Vector3,
    pub size: f32,
    pub x24: u32,
    pub joint: smash::phx::Hash40,
    pub shape: u8,
    pub shield_type: u8,
}

impl ShieldDataResource {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        size: f32,
        joint: smash::phx::Hash40,
        shape: u8,
        shield_type: u8
    ) -> Self {
        ShieldDataResource {
            offset: smash2::cpp::simd::Vector3{x: x, y: y, z: z},
            offset2: smash2::cpp::simd::Vector3{x: x2, y: y2, z: z2},
            size: size,
            x24: 0,
            joint: joint,
            shape: shape,
            shield_type: shield_type
        }
    }
}

//Shield Datas
#[repr(C)]
pub struct ShieldDatas {
    pub datas: [ShieldDataResource; 8]
}

impl ShieldDatas {
    pub fn new() -> ShieldDatas {
        ShieldDatas { datas: [
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0)
        ] }
    }
    pub fn add(mut self, shield_data: ShieldDataResource, index: usize) -> ShieldDatas {
        self.datas[index] = shield_data;
        self
    }
}

//Shield Group Resource
#[repr(C)]
pub struct ShieldGroupResource {
    pub shield_datas: *const ShieldDatas,
    pub count: u64,
    pub front: u8,
    pub hop: bool,
    pub turn: bool,
    pub no_hop: bool
}

impl ShieldGroupResource {
    pub fn new(
        shield_datas: *const ShieldDatas,
        count: u64,
        front: u8,
        hop: bool,
        turn: bool,
        no_hop: bool
    ) -> Self {
        ShieldGroupResource {
            shield_datas: shield_datas,
            count: count,
            front: front,
            hop: hop,
            turn: turn,
            no_hop: no_hop
        }
    }
}

//Adds a new shield type, used for making new counters
pub unsafe fn add_shield_group(boma: *mut BattleObjectModuleAccessor, resource: *mut ShieldGroupResource, group_id: i32) {
    let ptr = get_module_vtable_func(boma, 0x100, 0x58);
    let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource, i32) = std::mem::transmute(ptr);
    let shield_module = *(boma as *mut *mut u64).add(0x100/8);
    set_shield_group2(shield_module, resource, group_id);
    let count = (*resource).count as i32;
    if count > 0 {
        for x in 0..count {
            ShieldModule::set_status(boma, x, smash::app::ShieldStatus(*SHIELD_STATUS_NONE), group_id);
        }
    }
}

//Used to get the pointer for a vtable function within a specific module.
pub unsafe fn get_module_vtable_func(boma: *mut BattleObjectModuleAccessor, module_offset: usize, func_offset: u64) -> u64 {
    let module = (boma as *mut u64).add(module_offset/0x8);
    let vtable = *module as *const u64;
    *((*vtable + func_offset) as *const u64)
}

//Gets the boma of the grabbed opponent
pub unsafe fn get_grabbed_opponent_boma(attacker: *mut BattleObjectModuleAccessor) -> &'static mut BattleObjectModuleAccessor {
    let opponent_id = LinkModule::get_node_object_id(attacker, *LINK_NO_CAPTURE) as u32;
    let opponent_object = get_battle_object_from_id(opponent_id);
    return &mut *(*opponent_object).module_accessor
}