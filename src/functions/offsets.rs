extern "C" {
    #[link_name = "offsets_get_battle_object_from_id"]
    fn offsets_get_battle_object_from_id() -> usize;
}

pub fn get_battle_object_from_id() -> usize {
    unsafe {
        offsets_get_battle_object_from_id()
    }
}

#[cfg(not(feature = "no-offset-search"))]
mod offsets_impl {
    use crate::functions::util::byte_search;
    struct CoreOffsets {
        pub get_battle_object_from_id: usize
    }
    static GET_BATTLE_OBJECT_FROM_ID_SEARCH_CODE: &[u8] = &[
        0x1f, 0x60, 0x02, 0x39, // strb wzr, [x0, #0x98]
        0xc0, 0x03, 0x5f, 0xd6, // ret
        0x00, 0x00, 0x00, 0x00, // ??
        0x08, 0x7c, 0x1c, 0x53, // lsr w9, x0, #0x1C
        0x1f, 0x11, 0x00, 0x71, // cmp w8, #0x4
    ];
    const GET_BATTLE_OBJECT_FROM_ID_OFFSET_TO_START: usize = 0xC;
    lazy_static::lazy_static! {
        static ref CORE_OFFSETS: CoreOffsets = {
            let mut offsets = CoreOffsets {
                get_battle_object_from_id: 0
            };
            offsets.get_battle_object_from_id = byte_search(GET_BATTLE_OBJECT_FROM_ID_SEARCH_CODE).expect("Unable to find Item class constructor hook!") + GET_BATTLE_OBJECT_FROM_ID_OFFSET_TO_START;
            offsets
        };
    }
    #[export_name = "offsets_get_battle_object_from_id"]
    pub fn get_battle_object_from_id() -> usize {
        CORE_OFFSETS.get_battle_object_from_id
    }
}

pub use offsets_impl::*;