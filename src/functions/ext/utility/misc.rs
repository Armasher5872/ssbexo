#![allow(improper_ctypes)]
use super::*;

extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    fn get_dead_area() -> Rect;

    #[link_name = "_ZN3app10sv_animcmd25EFFECT_GLOBAL_BACK_GROUNDEP9lua_State"]
    fn effect_global_back_ground(lua_state: u64);
}

#[repr(simd)]
#[derive(Debug)]
struct Rect {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Rect {
    fn contains(&self, x: f32, y: f32) -> bool {
        (self.left <= x && x <= self.right) && (self.bottom <= y && y <= self.top)
    }
}

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

//Shield Data Resource, shieldboxes
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

//Shield Data, reflectorboxes

#[repr(C)]
pub struct ShieldData {
    pub offset: smash2::cpp::simd::Vector3,
    pub offset2: smash2::cpp::simd::Vector3,
    pub size: f32,
    pub x24: u32,
    pub joint: smash::phx::Hash40,
    pub shape: u8,
    pub shield_type: u8,
    pub x32: u16,
    pub x34: u32,
    pub x38: u64,
    pub status: i32,
    pub x44: u32,
    pub x48: u64,
}

impl ShieldData {
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
        ShieldData {
            offset: smash2::cpp::simd::Vector3{x: x, y: y, z: z},
            offset2: smash2::cpp::simd::Vector3{x: x2, y: y2, z: z2},
            size: size,
            x24: 0,
            joint: joint,
            shape: shape,
            shield_type: shield_type,
            x32: 0,
            x34: 0,
            x38: 0,
            status: 0,
            x44: 0,
            x48: 0,
        }
    }
}

//Shield Datas, shieldboxes
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

//Shield Datas 2, reflectors
#[repr(C)]
pub struct ShieldDatas2 {
    pub datas: [ShieldData; 8]
}

impl ShieldDatas2 {
    pub fn new() -> ShieldDatas2 {
        ShieldDatas2 { datas: [
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0),
            ShieldData::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, smash::phx::Hash40::new_raw(0), 0, 0)
        ] }
    }
    pub fn add(mut self, shield_data: ShieldData, index: usize) -> ShieldDatas2 {
        self.datas[index] = shield_data;
        self
    }
}

//Shield Group Resource, shieldboxes
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

//Shield Group Resource 2, reflectors
#[repr(C)]
pub struct ShieldGroupResource2 {
    pub shield_datas: *const ShieldDatas2,
    pub count: u64,
    pub attack_mul: f32,
    pub speed_mul: f32,
    pub attack_limit: f32,
    pub life_mul: f32,
    pub no_m_ball: bool,
    pub front: u8
}

impl ShieldGroupResource2 {
    pub fn new(
        shield_datas: *const ShieldDatas2,
        count: u64,
        attack_mul: f32,
        speed_mul: f32,
        attack_limit: f32,
        life_mul: f32,
        no_m_ball: bool,
        front: u8
    ) -> Self {
        ShieldGroupResource2 {
            shield_datas: shield_datas,
            count: count,
            attack_mul: attack_mul,
            speed_mul: speed_mul,
            attack_limit: attack_limit,
            life_mul: life_mul,
            no_m_ball: no_m_ball,
            front: front
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

//Adds a new reflector type, used for making new reflectors
pub unsafe fn add_reflector_group(boma: *mut BattleObjectModuleAccessor, resource: *mut ShieldGroupResource2, group_id: i32) {
    let ptr = get_module_vtable_func(boma, 0x108, 0x60);
    let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource2, i32) = std::mem::transmute(ptr);
    let reflector_module = *(boma as *mut *mut u64).add(0x108/8);
    set_shield_group2(reflector_module, resource, group_id);
    let count = (*resource).count as i32;
    if count > 0 {
        for x in 0..count {
            ReflectorModule::set_status(boma, x, smash::app::ShieldStatus(*SHIELD_STATUS_NONE), group_id);
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

//Credited to HDR, used for adding 0 values of Vector2f and Vector3f

pub trait Vec2Ext {
    fn new(x: f32, y: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
}

pub trait Vec3Ext {
    fn new(x: f32, y: f32, z: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
}

impl Vec2Ext for smash::phx::Vector2f {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Vec3Ext for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

//Credited to HDR, used for calculating Finishing Zoom

#[derive(Debug, Copy, Clone)]
pub struct KnockbackCalcContext {
    pub knockback: f32,
    pub x_launch_speed: f32,
    pub y_launch_speed: f32,
    pub y_chara_speed: f32,
    pub tumble: bool,
    pub is_damage_fly_top: bool,
    pub hitstun: f32,
    pub gravity: f32,
    pub damageflytop_gravity: f32,
    pub fall_speed: f32,
    pub damageflytop_fall_speed: f32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub decay_x: f32,
    pub decay_y: f32,
}

impl KnockbackCalcContext {
    pub fn step(&mut self) {
        self.x_pos += self.x_launch_speed;
        self.y_pos += self.y_launch_speed + self.y_chara_speed;
        if self.x_launch_speed != 0.0 {
            let dir = self.x_launch_speed.signum();
            self.x_launch_speed = self.x_launch_speed.abs() - self.decay_x;
            if self.x_launch_speed < 0.0 {
                self.x_launch_speed = 0.0;
            } 
            else {
                self.x_launch_speed *= dir;
            }
        }
        if self.y_launch_speed != 0.0 {
            let dir = self.y_launch_speed.signum();
            self.y_launch_speed = self.y_launch_speed.abs() - self.decay_y;
            if self.y_launch_speed < 0.0 {
                self.y_launch_speed = 0.0;
            } 
            else {
                self.y_launch_speed *= dir;
            }
        }
        if self.is_damage_fly_top {
            self.y_chara_speed -= self.damageflytop_gravity;
            if self.y_chara_speed < -self.damageflytop_fall_speed {
                self.y_chara_speed = -self.damageflytop_fall_speed;
            }
        } 
        else {
            self.y_chara_speed -= self.gravity;
            if self.y_chara_speed < -self.fall_speed {
                self.y_chara_speed = -self.fall_speed;
            }
        }
    }
}

/// Knockback log
/// 0x8a -> the opponent was grounded (bool)
/// 0x90 -> backslash (bool)
/// 0x60 -> stop delay (f32) 
/// 0x50 -> collision attr (Hash40)
/// 0x40 -> launch angle in rad (f32)
/// 0x4 -> level (?)
/// 0x4c -> hitstop frame
pub unsafe extern "C" fn calculate_finishing_hit(defender: u32, attacker: u32, knockback_info: *const f32) {
    *(knockback_info.add(0x4c/4) as *mut u32) = 60;
    let defender_boma = &mut *(*get_battle_object_from_id(defender)).module_accessor;
    let attacker_boma = &mut *(*get_battle_object_from_id(attacker)).module_accessor;
    if !defender_boma.is_fighter() { 
        return; 
    }
    if !attacker_boma.is_fighter() && !attacker_boma.is_weapon() { 
        return; 
    }
    /*
    if !is_training_mode() {
        // ensure kill calculations only occur when the defender is on their last stock
        let entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_info = smash::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), FighterEntryID(entry_id));
        if smash::app::lua_bind::FighterInformation::stock_count(fighter_info) != 1 { 
            WorkModule::off_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK);
        }
        else {
            WorkModule::on_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK);
        }
    }
    */
    if is_invalid_finishing_hit(attacker_boma) {
        return;
    }
    if smash2::app::FighterCutInManager::is_vr_mode() {
        return;
    }
    if !FighterCutInManager__is_one_on_one() {
        return;
    }
    // ensure kill calculations only occur when the defender is on their last stock
    let entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_info = smash::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), FighterEntryID(entry_id));
    if smash::app::lua_bind::FighterInformation::stock_count(fighter_info) != 1 { 
        WorkModule::off_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK);
    }
    else {
        WorkModule::on_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK);
    }
    let knockback = *knockback_info;
    let initial_speed_x = *knockback_info.add(4);
    let initial_speed_y = *knockback_info.add(5);
    let reaction = *knockback_info.add(0x48 / 4);
    let angle = *knockback_info.add(0x10);
    let counter = WorkModule::get_int(defender_boma, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let fly_top_angle_hi = WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_lw"));
    let fly_top_angle_lw = WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_hi"));
    let damage_air_brake = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("damage_air_brake"));
    let damage_fly_correction_max = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("damage_fly_correction_max"));
    let air_accel_y = WorkModule::get_param_float(defender_boma, hash40("air_accel_y"), 0);
    let damage_fly_top_air_accel_y = WorkModule::get_param_float(defender_boma, hash40("damage_fly_top_air_accel_y"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(defender_boma, hash40("air_speed_y_stable"), 0);
    let damage_fly_top_speed_y_stable = WorkModule::get_param_float(defender_boma, hash40("damage_fly_top_speed_y_stable"), 0);
    let mut context = KnockbackCalcContext{knockback, x_launch_speed: initial_speed_x, y_launch_speed: initial_speed_y, y_chara_speed: 0.0, tumble: *(knockback_info.add(1) as *const u32) >= 3, is_damage_fly_top: fly_top_angle_lw <= angle && angle <= fly_top_angle_hi, hitstun: reaction, gravity: air_accel_y, damageflytop_gravity: damage_fly_top_air_accel_y, fall_speed: air_speed_y_stable, damageflytop_fall_speed: damage_fly_top_speed_y_stable, x_pos: PostureModule::pos_x(defender_boma), y_pos: PostureModule::pos_y(defender_boma), decay_x: damage_air_brake*angle.cos().abs(), decay_y: damage_air_brake*angle.sin().abs()};
    let blastzones = get_dead_area();
    let mag = (context.y_launch_speed.powi(2)+context.x_launch_speed.powi(2)).sqrt();
    let kb_angle = context.y_launch_speed.atan2(context.x_launch_speed).to_degrees();
    let min_di = kb_angle-damage_fly_correction_max;
    let step = (damage_fly_correction_max*2.0)/10.0;
    let context_ref = context;
    //checks 10 different DI angles and sees how many of them will result in a kill
    let mut kill_angle_num = 0;
    for idx in 0..10 {
        let ang = (min_di+(idx as f32*step)).to_radians();
        context.x_launch_speed = ang.cos()*mag;
        context.y_launch_speed = ang.sin()*mag;
        let mut x = 0;
        let mut will_touch_stage = false;
        while context.hitstun > x as f32 {
            context.step();
            if GroundModule::ray_check(defender_boma, &Vector2f{x: context.x_pos, y: (context.y_pos + 4.0)}, &Vector2f{x: 0.0, y: -6.0}, true) == 1 && !(30.0..150.0).contains(&ang.to_degrees()) {
                will_touch_stage = true;
            }
            if !blastzones.contains(context.x_pos, context.y_pos) && !will_touch_stage {
                kill_angle_num += 1;
                break;
            }
            x += 1;
        }
        context = context_ref;
    }
    if kill_angle_num >= 9 && counter == 0 {
        let attacker_kind = (*attacker_boma).kind();
        if WorkModule::is_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
            let handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_finishhit"), false, true, true);
            EffectModule::set_billboard(attacker_boma, handle as u32, true);
            EffectModule::set_disable_render_offset_last(attacker_boma);
            WorkModule::set_int(attacker_boma, 50, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
            SoundModule::play_se(defender_boma, Hash40::new("se_common_finishhit"), false, false, false, false, enSEType(0));
            SlowModule::set_whole(defender_boma, 8, 50);
            macros::CAM_ZOOM_IN_arg5(get_fighter_common_from_accessor(&mut *defender_boma), /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
            macros::QUAKE(get_fighter_common_from_accessor(&mut *defender_boma), *CAMERA_QUAKE_KIND_XL);
            effect_global_back_ground(get_fighter_common_from_accessor(&mut *defender_boma).lua_state_agent);
        }
        else {
            let handle = match attacker_kind {
                _ if attacker_kind == *FIGHTER_KIND_MARIO => EffectModule::req_screen(attacker_boma, Hash40::new("bg_mario_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_DONKEY => EffectModule::req_screen(attacker_boma, Hash40::new("bg_donkey_final"), false, true, true),
                _ if [*FIGHTER_KIND_LINK, *WEAPON_KIND_LINK_SWORD_BEAM, *WEAPON_KIND_LINK_BOWARROW, *WEAPON_KIND_LINK_BOOMERANG].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_link_final"), false, true, true),
                _ if [*FIGHTER_KIND_SAMUS, *WEAPON_KIND_SAMUS_CSHOT, *WEAPON_KIND_SAMUS_MISSILE, *WEAPON_KIND_SAMUS_SUPERMISSILE, *WEAPON_KIND_SAMUS_BOMB].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_samus_final"), false, true, true),
                _ if [*FIGHTER_KIND_SAMUSD, *WEAPON_KIND_SAMUSD_CSHOT, *WEAPON_KIND_SAMUSD_MISSILE, *WEAPON_KIND_SAMUSD_SUPERMISSILE, *WEAPON_KIND_SAMUSD_BOMB].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_samusd_final"), false, true, true),
                _ if [*FIGHTER_KIND_YOSHI, *WEAPON_KIND_YOSHI_TAMAGO, *WEAPON_KIND_YOSHI_STAR].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_yoshi_final"), false, true, true),
                _ if [*FIGHTER_KIND_KIRBY, *WEAPON_KIND_KIRBY_HAMMER, *WEAPON_KIND_KIRBY_FINALCUTTERSHOT, *WEAPON_KIND_KIRBY_ROSETTATICOMISSILE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_kirby_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_PIKACHU => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pikachu_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_LUIGI => EffectModule::req_screen(attacker_boma, Hash40::new("bg_luigi_final"), false, true, true),
                _ if [*FIGHTER_KIND_NESS, *WEAPON_KIND_NESS_YOYO_HEAD, *WEAPON_KIND_NESS_PK_FLASH, *WEAPON_KIND_NESS_PK_FIRE, *WEAPON_KIND_NESS_PK_THUNDER].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ness_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_CAPTAIN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_captain_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_PURIN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_purin_final"), false, true, true),
                _ if [*FIGHTER_KIND_PEACH, *WEAPON_KIND_PEACH_KINOPIOSPORE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_peach_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_DAISY => EffectModule::req_screen(attacker_boma, Hash40::new("bg_daisy_final"), false, true, true),
                _ if [*FIGHTER_KIND_KOOPA, *WEAPON_KIND_KOOPA_BREATH].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_koopa_final"), false, true, true),
                _ if [*FIGHTER_KIND_POPO, *WEAPON_KIND_POPO_ICESHOT].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_popo_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_NANA => EffectModule::req_screen(attacker_boma, Hash40::new("bg_popo_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_SHEIK => EffectModule::req_screen(attacker_boma, Hash40::new("bg_sheik_final"), false, true, true),
                _ if [*FIGHTER_KIND_ZELDA, *WEAPON_KIND_ZELDA_DEIN, *WEAPON_KIND_ZELDA_DEIN_S, *WEAPON_KIND_ZELDA_PHANTOM].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_zelda_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_MARIOD => EffectModule::req_screen(attacker_boma, Hash40::new("bg_mariod_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_PICHU => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pichu_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_MARTH => EffectModule::req_screen(attacker_boma, Hash40::new("bg_marth_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_LUCINA => EffectModule::req_screen(attacker_boma, Hash40::new("bg_lucina_final"), false, true, true),
                _ if [*FIGHTER_KIND_YOUNGLINK, *WEAPON_KIND_YOUNGLINK_BOWARROW, *WEAPON_KIND_YOUNGLINK_HOOKSHOT_HAND, *WEAPON_KIND_YOUNGLINK_BOOMERANG].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_younglink_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_GANON => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ganon_final"), false, true, true),
                _ if [*FIGHTER_KIND_MEWTWO, *WEAPON_KIND_MEWTWO_BINDBALL].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_mewtwo_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_ROY => EffectModule::req_screen(attacker_boma, Hash40::new("bg_roy_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_CHROM => EffectModule::req_screen(attacker_boma, Hash40::new("bg_chrom_final"), false, true, true),
                _ if [*FIGHTER_KIND_GAMEWATCH, *WEAPON_KIND_GAMEWATCH_NORMAL_WEAPON, *WEAPON_KIND_GAMEWATCH_PARACHUTE, *WEAPON_KIND_GAMEWATCH_BREATH, *WEAPON_KIND_GAMEWATCH_RESCUE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_gamewatch_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_METAKNIGHT => EffectModule::req_screen(attacker_boma, Hash40::new("bg_metaknight_final"), false, true, true),
                _ if [*FIGHTER_KIND_PIT, *WEAPON_KIND_PIT_BOWARROW].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pit_final"), false, true, true),
                _ if [*FIGHTER_KIND_PITB, *WEAPON_KIND_PITB_BOWARROW].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pitb_final"), false, true, true),
                _ if [*FIGHTER_KIND_SZEROSUIT, *WEAPON_KIND_SZEROSUIT_PARALYZER_BULLET, *WEAPON_KIND_SZEROSUIT_WHIP, *WEAPON_KIND_SZEROSUIT_WHIP2].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_szerosuit_final"), false, true, true),
                _ if [*FIGHTER_KIND_WARIO, *WEAPON_KIND_WARIO_WARIOBIKE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_wario_final"), false, true, true),
                _ if [*FIGHTER_KIND_SNAKE, *WEAPON_KIND_SNAKE_C4, *WEAPON_KIND_SNAKE_NIKITA_MISSILE, *WEAPON_KIND_SNAKE_TRENCHMORTAR_BULLET].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_snake_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_IKE => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ike_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_PZENIGAME => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ptrainer_final"), false, true, true),
                _ if [*FIGHTER_KIND_PFUSHIGISOU, *WEAPON_KIND_PFUSHIGISOU_LEAFCUTTER, *WEAPON_KIND_PFUSHIGISOU_VINE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ptrainer_final"), false, true, true),
                _ if [*FIGHTER_KIND_PLIZARDON, *WEAPON_KIND_PLIZARDON_EXPLOSION].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ptrainer_final"), false, true, true),
                _ if [*FIGHTER_KIND_DIDDY, *WEAPON_KIND_DIDDY_PEANUTS, *WEAPON_KIND_DIDDY_EXPLOSION].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_diddy_final"), false, true, true),
                _ if [*FIGHTER_KIND_LUCAS, *WEAPON_KIND_LUCAS_PK_FREEZE, *WEAPON_KIND_LUCAS_PK_FIRE, *WEAPON_KIND_LUCAS_PK_THUNDER].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_lucas_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_SONIC => EffectModule::req_screen(attacker_boma, Hash40::new("bg_sonic_final"), false, true, true),
                _ if [*FIGHTER_KIND_DEDEDE, *WEAPON_KIND_DEDEDE_GORDO, *WEAPON_KIND_DEDEDE_STAR].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_dedede_final"), false, true, true),
                _ if [*FIGHTER_KIND_PIKMIN, *WEAPON_KIND_PIKMIN_PIKMIN].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pikmin_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_LUCARIO => EffectModule::req_screen(attacker_boma, Hash40::new("bg_lucario_final"), false, true, true),
                _ if [*FIGHTER_KIND_ROBOT, *WEAPON_KIND_ROBOT_BEAM].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_robot_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_TOONLINK => EffectModule::req_screen(attacker_boma, Hash40::new("bg_toonlink_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_MURABITO => EffectModule::req_screen(attacker_boma, Hash40::new("bg_murabito_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_ROCKMAN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_rockman_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_WIIFIT => EffectModule::req_screen(attacker_boma, Hash40::new("bg_wiifit_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_ROSETTA => EffectModule::req_screen(attacker_boma, Hash40::new("bg_rosetta_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_LITTLEMAC => EffectModule::req_screen(attacker_boma, Hash40::new("bg_littlemac_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_GEKKOUGA => EffectModule::req_screen(attacker_boma, Hash40::new("bg_gekkouga_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_PALUTENA => EffectModule::req_screen(attacker_boma, Hash40::new("bg_palutena_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_PACMAN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pacman_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_REFLET => EffectModule::req_screen(attacker_boma, Hash40::new("bg_reflet_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_SHULK => EffectModule::req_screen(attacker_boma, Hash40::new("bg_shulk_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_KOOPAJR => EffectModule::req_screen(attacker_boma, Hash40::new("bg_koopajr_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_DUCKHUNT => EffectModule::req_screen(attacker_boma, Hash40::new("bg_duckhunt_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_RYU => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ryu_final_shinsyoryu"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_KEN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ken_final_shinryuken"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_CLOUD => EffectModule::req_screen(attacker_boma, Hash40::new("bg_cloud_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_KAMUI => EffectModule::req_screen(attacker_boma, Hash40::new("bg_kamui_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_BAYONETTA => EffectModule::req_screen(attacker_boma, Hash40::new("bg_bayonetta_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_INKLING => EffectModule::req_screen(attacker_boma, Hash40::new("bg_inkling_final_l"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_RIDLEY => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ridley_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_SIMON => EffectModule::req_screen(attacker_boma, Hash40::new("bg_simon_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_RICHTER => EffectModule::req_screen(attacker_boma, Hash40::new("bg_richter_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_KROOL => EffectModule::req_screen(attacker_boma, Hash40::new("bg_krool_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_SHIZUE => EffectModule::req_screen(attacker_boma, Hash40::new("bg_shizue_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_GAOGAEN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_gaogaen_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_PACKUN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_packun_final1"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_JACK => EffectModule::req_screen(attacker_boma, Hash40::new("bg_jack_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_BRAVE => EffectModule::req_screen(attacker_boma, Hash40::new("bg_brave_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_BUDDY => EffectModule::req_screen(attacker_boma, Hash40::new("bg_buddy_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_DOLLY => EffectModule::req_screen(attacker_boma, Hash40::new("bg_dolly_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_MASTER => EffectModule::req_screen(attacker_boma, Hash40::new("bg_master_final_l"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_TANTAN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_tantan_final_l"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_PICKEL => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pickel_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_EDGE => EffectModule::req_screen(attacker_boma, Hash40::new("bg_edge_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_EFLAME => EffectModule::req_screen(attacker_boma, Hash40::new("bg_eflame_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_ELIGHT => EffectModule::req_screen(attacker_boma, Hash40::new("bg_eelight_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_DEMON => EffectModule::req_screen(attacker_boma, Hash40::new("bg_demon_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_TRAIL => EffectModule::req_screen(attacker_boma, Hash40::new("bg_trail_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_MIIFIGHTER => EffectModule::req_screen(attacker_boma, Hash40::new("bg_miifighter_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_MIISWORDSMAN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_miiswordsman_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_MIIGUNNER => EffectModule::req_screen(attacker_boma, Hash40::new("bg_miigunner_final"), false, true, true),
                _ => EffectModule::req_screen(attacker_boma, Hash40::new("bg_criticalhit"), false, true, true)
            };
            EffectModule::set_billboard(attacker_boma, handle as u32, true);
            EffectModule::set_disable_render_offset_last(attacker_boma);
            EffectModule::set_rate(attacker_boma, handle as u32, 8.0);
            WorkModule::set_int(attacker_boma, 50, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
            WorkModule::set_int(attacker_boma, handle as i32, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
            SoundModule::play_se(defender_boma, Hash40::new("se_common_boss_down"), false, false, false, false, enSEType(0));
            SlowModule::set_whole(defender_boma, 8, 50);
            effect_global_back_ground(get_fighter_common_from_accessor(&mut *defender_boma).lua_state_agent);
        }
        match attacker_kind {
            _ if attacker_kind == *FIGHTER_KIND_PZENIGAME => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_ptrainer_win_pzenigame")),
            _ if attacker_kind == *FIGHTER_KIND_PFUSHIGISOU => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_ptrainer_win_pfushigisou")),
            _ if attacker_kind == *FIGHTER_KIND_PLIZARDON => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_ptrainer_win_plizardon")),
            _ if attacker_kind == *FIGHTER_KIND_LITTLEMAC => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_littlemac_win_dl06")),
            _ if attacker_kind == *FIGHTER_KIND_REFLET => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_reflet_final_chrom09")),
            _ if attacker_kind == *FIGHTER_KIND_JACK => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_jack_appeal01")),
            _ if attacker_kind == *FIGHTER_KIND_EFLAME => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_eflame_diver_apeal01")),
            _ if attacker_kind == *FIGHTER_KIND_ELIGHT => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_elight_diver_apeal01")),
            _ => {}
        };
    }
}

unsafe extern "C" fn is_invalid_finishing_hit(boma: &mut BattleObjectModuleAccessor) -> bool {
    for id in 0..8 {
        let attack_data = AttackModule::attack_data(boma, id, false);
        let attribute = (*attack_data).attr;
        if [hash40("collision_attr_saving"), hash40("collision_attr_lay"), hash40("collision_attr_bury")].contains(&attribute) {
            return true;
        }
    }
    return false;
}