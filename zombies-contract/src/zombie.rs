multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Zombie<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub dna: u64,
}
