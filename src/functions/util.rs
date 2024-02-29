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

pub fn byte_search<T: Eq>(needle: &[T]) -> Option<usize> {   
    let text = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
        let end = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
        let length = end.offset_from(start) as usize;
        std::slice::from_raw_parts(start, length)
    };

    text.windows(needle.len()).position(|window| window == needle)
}

pub fn byte_search_rodata<T: Eq>(needle: &[T]) -> Option<usize> {
    const RODATA_LEN: usize = 0xCC8C9B;
    let (rodata, text_len) = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
        let end = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as usize + RODATA_LEN) as *const T;
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
        let length = end.offset_from(start) as usize;
        (std::slice::from_raw_parts(start, length), start.offset_from(text) as usize)
    };

    rodata.windows(needle.len()).position(|window| window == needle).map(|x| x + text_len)
}

pub fn offset_to_addr<T>(offset: usize) -> *const T {
    unsafe {
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8).add(offset) as _
    }
}

#[skyline::from_offset(crate::functions::offsets::get_battle_object_from_id())]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

pub fn get_battle_object_from_accessor(boma: *mut BattleObjectModuleAccessor) -> *mut BattleObject {
    unsafe {
        get_battle_object_from_id((*boma).battle_object_id)
    }
}

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

pub fn get_fighter_common_from_entry_id(entry_id: u32) -> Option<&'static mut L2CFighterCommon> {
    if let Some(object) = get_battle_object_from_entry_id(entry_id) {
        unsafe {
            Some(get_fighter_common_from_accessor(std::mem::transmute((*object).module_accessor)))
        }
    } else {
        None
    }
}

pub fn get_lua_state_from_entry_id(entry_id: u32) -> Option<u64> {
    get_fighter_common_from_entry_id(entry_id).map(|x| x.lua_state_agent)
}

pub fn get_active_battle_object_id_from_entry_id(entry_id: u32) -> Option<u32> {
    use smash::lib::lua_const::*;
    use smash::app::lua_bind::*;
    let object = get_battle_object_from_entry_id(entry_id)?;
    if object.is_null() { return None; }
    let object = unsafe { &mut *object };
    let kind = object.kind as i32;
    let status = unsafe {
        StatusModule::status_kind(object.module_accessor)
    };
    if status != *FIGHTER_STATUS_KIND_NONE && status != *FIGHTER_STATUS_KIND_STANDBY {
        return Some(object.battle_object_id);
    }
    if kind == *FIGHTER_KIND_ELIGHT || kind == *FIGHTER_KIND_EFLAME {
        Some(object.battle_object_id + 0x10000)
    } 
    else if kind == *FIGHTER_KIND_PZENIGAME || kind == *FIGHTER_KIND_PFUSHIGISOU || kind == *FIGHTER_KIND_PLIZARDON {
        let next_id = object.battle_object_id + 0x10000;
        let next_object = unsafe { &mut *get_battle_object_from_id(next_id) };
        let next_status = unsafe {
            StatusModule::status_kind(next_object.module_accessor)
        };
        if next_status != *FIGHTER_STATUS_KIND_NONE && next_status != *FIGHTER_STATUS_KIND_STANDBY {
            Some(next_id)
        } 
        else {
            Some(next_id + 0x10000)
        }
    } 
    else {
        Some(object.battle_object_id)
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;
}

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(crate::functions::singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}

/* 
Utility function to compare two masks, such as a "cat flag". Internally, its just a binary & operation. When possible, use fighter.is_cat_flag() or boma.is_cat_flag() instead, for clarity.

# Arguments: 

* `mask1` : a bitmask, such as a cat flag mask (cat1, cat2, etc)\n
* `mask2` : another bitmask, such as `*FIGHTER_KIND_MARIO`

# Returns: `bool` : whether the binary & of the masks is nonzero (aka "true")
*/
pub fn compare_mask(mask1: i32, mask2: i32) -> bool {
    return (mask1 & mask2) != 0;
}

/*
Get a vector3f where the given value is the strength and stick X is used to determine the direction of the vector in the x direction. y and z are always zero.
*/
#[inline(always)]
pub unsafe fn x_motion_vec(val: f32, stick_x: f32) -> smash::phx::Vector3f {
    smash::phx::Vector3f{x: val * stick_x.signum(), y: 0.0, z: 0.0}
}

extern "C"{
    /// gets whether we are in training mode
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
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

pub unsafe fn get_player_idx_from_boma(boma: u64) -> i32 {
    let control_module = *((boma + 0x48) as *const u64);
    let next = *((control_module + 0x118) as *const u64);
    let next = *((next + 0x58) as *const u64);
    let next = *((next + 0x8) as *const u64);
    *((next + 0x8) as *const i32)
}

pub unsafe fn get_mapped_controller_inputs_from_id(player: usize) -> &'static MappedInputs {
    let base = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x52c50f0) as *const u64);
    &*((base + 0x2b8 + 0x8 * (player as u64)) as *const MappedInputs)
}