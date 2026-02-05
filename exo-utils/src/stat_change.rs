//Credited to WuBoyTH
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StatChangeGroup {
    pub group: *mut StatChange,
    pub end: *mut StatChange,
    pub end2: *mut StatChange,
}

impl StatChangeGroup {
    pub fn empty(capacity: usize) -> *mut StatChangeGroup {
        let layout = std::alloc::Layout::array::<StatChange>(capacity).unwrap();
        let ptr = unsafe { 
            let raw_ptr = std::alloc::alloc_zeroed(layout) as *mut StatChange;
            if raw_ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            raw_ptr
        };
        Box::into_raw(Box::new(StatChangeGroup {
            group: ptr,
            end: ptr, // Start with same pointer (empty array)
            end2: ptr,
        }))
    }
    pub fn new(changes: Vec<StatChange>) -> *mut StatChangeGroup {
        let mut changes = changes;
        let ptr = changes.as_mut_ptr();
        let len = changes.len();
        std::mem::forget(changes); // Prevent Rust from dropping
        // Calculate end pointer (first element + length)
        let end_ptr = unsafe { ptr.add(len) };
        let group = Box::new(StatChangeGroup {
            group: ptr,
            end: end_ptr,
            end2: end_ptr
        });
        Box::into_raw(group)
    }
}

#[repr(C)]
pub struct StatChange {
    pub param_hash: u64,
    pub padding: u64,
    pub some: f32,
    pub mul: f32,
    pub some2: f32,
    pub some3: f32,
    pub padding3: u32,
    pub padding4: u32,
}

impl StatChange {
    pub fn new(param_hash: u64, mul: f32) -> StatChange {
        StatChange {
            param_hash,
            padding: 0,
            some: 1.0,
            mul,
            some2: 0.0,
            some3: 1.0,
            padding3: 0,
            padding4: 0
        }
    }
}