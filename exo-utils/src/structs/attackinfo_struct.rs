//Attacker Info, used to get the attackers stats
#[repr(C)]
pub struct AttackerInfo {
    pub attacker_id: u32,
    pub attacker_category: u8,
    //undefined3
    pub attacker_kind: i32,
    pub indirect_id: u32,
    pub indirect_category: u8,
    //undefined3
    pub indirect_kind: i32,
    pub attacker_entry_id: u32,
    pub team_owner_id: u32,
    pub metamon_owner_id: u32,
    pub metamon_entry_id: u32
}