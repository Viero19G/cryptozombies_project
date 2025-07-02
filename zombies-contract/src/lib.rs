#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;
mod zombie;
mod zombie_factory;
mod zombie_feeding;



#[multiversx_sc::contract]
pub trait ZombiesContract:
    zombie_factory::ZombieFactory
    + zombie_feeding::ZombieFeeding
    + storage::Storage
{
    #[init]
    fn init(&self) {
        self.dna_digits().set(16u8);
        self.zombie_last_index().set(1usize);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
