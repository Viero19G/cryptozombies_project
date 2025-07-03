multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{storage, zombie_factory};

#[multiversx_sc::module]
pub trait ZombieFeeding:
    storage::Storage
    + zombie_factory::ZombieFactory
{
    #[endpoint]
    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64) {
        let caller = self.blockchain().get_caller(); // Obtém o endereço de quem chamou esta função.

        // LINHA EXPLICADA: require!(caller == self.zombie_owner(&zombie_id).get(), "Only the owner of the zombie can perform this operation");
        // O que faz: Esta linha é um "portão de segurança". Ela verifica se uma condição é verdadeira.
        // Se a condição for FALSA, a execução da transação é PARADA IMEDIATAMENTE 
        
        // A condição aqui é: `caller == self.zombie_owner(&zombie_id).get()`
        // - `caller`: É o endereço da carteira ou contrato que está tentando executar esta função.
        // - `self.zombie_owner(&zombie_id).get()`: Obtém o endereço do proprietário registrado do zumbi
        //   com o `zombie_id` fornecido.

        // POR QUE EXISTE:
        //  Garante que apenas o proprietário legítimo do zumbi possa realizar esta operação.
        // Se a condição for falsa, a mensagem "Only the owner of the zombie can perform this operation"
        // será retornada como erro.
        require!(
            caller == self.zombie_owner(&zombie_id).get(),
            "Only the owner of the zombie can perform this operation"
        );
        let my_zombie = self.zombies(&zombie_id).get(); // Se o "require!" passar, o código continua aqui.
    }
}
