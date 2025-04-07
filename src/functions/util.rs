//Credited to HDR
use super::*;

#[macro_export]
macro_rules! dump_trace {
    () => {{
        let text = ::skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        ::utils::dump_trace!(text)
    }};
    ($base:expr) => {{
        const MAXIMUM_BT_LEN: usize = 0x20;
        let text = $base;
        println!("Current text address: {:#x}", text);

        let mut lr: *const u64;
        unsafe {
            asm!("mov {}, x30", out(reg) lr);
        }

        let mut fp: *const u64;
        unsafe {
            asm!("mov {}, x29", out(reg) fp);
        }

        println!("Current LR:\t\t{:#x} ({:#x})", (lr as u64) - text, (lr as u64));

        let mut counter = 0usize;
        while !fp.is_null() && counter < MAXIMUM_BT_LEN {
            lr = *fp.offset(1) as *const u64;
            if !lr.is_null() {
                println!("[{}]: {:#x} ({:#x})", counter, (lr as u64), (lr as u64) - text);
                counter += 1;
            }
            fp = *fp as *const u64;
        }
    }}
}

#[macro_export]
macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    }
}

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

pub fn get_weapon_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CWeaponCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CWeaponCommon))
    }
}

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } 
        else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}

#[repr(C)]
pub struct HashedString {
    pub hash: smash::phx::Hash40,
    pub contents: [u8; 0x100]
}

#[repr(C)]
pub struct CppVector<T> {
    start: *mut T,
    end: *mut T,
    eos: *mut T
}

impl<T> CppVector<T> {
    pub fn len(&self) -> usize {
        unsafe {
            self.end.offset_from(self.start) as usize
        }
    }
}

impl<T: Copy> CppVector<T> {
    pub fn push(&mut self, value: T) {
        unsafe {
            let length = self.end.offset_from(self.start) as usize;
            let cap = self.eos.offset_from(self.start) as usize;
            if length == cap {
                let new_ptr = skyline::libc::malloc(std::mem::size_of::<T>() * cap * 2);
                skyline::libc::memcpy(new_ptr, self.start as _, std::mem::size_of::<T>() * length);
                let old = self.start;
                self.start = new_ptr as _;
                self.end = self.start.add(length as usize);
                self.eos = self.start.add((cap * 2) as usize);

                skyline::libc::free(old as _);
            }

            *self.end = value;
            self.end = self.end.add(1);
        }
    }
}

pub unsafe fn get_mapped_controller_inputs_from_id(player: usize) -> &'static MappedInputs {
    let base = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x52c50f0) as *const u64);
    &*((base + 0x2b8 + 0x8 * (player as u64)) as *const MappedInputs)
}

pub fn get_active_battle_object_id_from_entry_id(entry_id: u32) -> Option<u32> {
    let object = get_battle_object_from_entry_id(entry_id)?;
    if object.is_null() {return None;}
    let object = unsafe {&mut *object};
    let kind = object.kind as i32;
    let status = unsafe {StatusModule::status_kind(object.module_accessor)};
    if ![*FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_STANDBY].contains(&status) {
        return Some(object.battle_object_id);
    }
    if [*FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_EFLAME].contains(&kind) {
        Some(object.battle_object_id+0x10000)
    } 
    else if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU,  *FIGHTER_KIND_PLIZARDON].contains(&kind) {
        let next_id = object.battle_object_id+0x10000;
        let next_object = unsafe {get_battle_object_from_id(next_id)};
        if !next_object.is_null() {
            let next_object = unsafe {&mut *next_object};
            let next_status = unsafe {StatusModule::status_kind(next_object.module_accessor)};
            if ![*FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_STANDBY].contains(&next_status) {
                Some(next_id)
            } 
            else {
                Some(next_id+0x10000)
            }
        }
        else {
            Some(object.battle_object_id)
        }
    } 
    else {
        Some(object.battle_object_id)
    }
}

//Obtains all active battle_object_ids, including both Ice Climbers and only the active character of Pokemon Trainer and Aegis
pub unsafe fn get_all_active_battle_object_ids() -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for entry_id in 0..8 {
        let id = get_active_battle_object_id_from_entry_id(entry_id).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
        vec.push(id);
        let object = get_battle_object_from_id(id);
        if object.is_null() {continue;}
        let object = unsafe {&mut *object};
        let kind = object.kind as i32;
        if kind != *FIGHTER_KIND_POPO {continue;}
        let boma = &mut *(*object).module_accessor;
        let nana_id = WorkModule::get_int(boma, *FIGHTER_POPO_INSTANCE_WORK_ID_INT_PARTNER_OBJECT_ID) as u32;
        let nana_object = get_battle_object_from_id(nana_id);
        if nana_object.is_null() {continue;}
        let nana_object = unsafe {&mut *nana_object};
        vec.push(nana_object.battle_object_id);
    }
    return vec;
}