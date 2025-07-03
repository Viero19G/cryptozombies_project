use multiversx_sc::derive_imports::*;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Kitty {
    pub genes: KittyGenes,
    pub birth_time: u64,  
    pub cooldown_end: u64,
    pub matron_id: u32,
    pub sire_id: u32,
    pub siring_with_id: u32,
    pub nr_children: u16,   
    pub generation: u16,    
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct KittyGenes {
    pub fur_color: Color,
    pub eye_color: Color,
    pub meow_power: u8,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
