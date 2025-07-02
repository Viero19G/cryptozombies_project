multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait Storage {
    #[storage_mapper("dnaDigits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("zombieLastIndex")]
    fn zombie_last_index(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("zombies")]
    fn zombies(&self, id: &usize) -> SingleValueMapper<Zombie<Self::Api>>;

    #[storage_mapper("zombieOwner")]
    fn zombie_owner(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("ownedZombies")]
    fn owned_zombies(&self, owner: &ManagedAddress) -> UnorderedSetMapper<usize>;
}
