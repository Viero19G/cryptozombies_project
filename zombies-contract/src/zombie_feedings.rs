multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{kitty_obj::Kitty, kitty_ownership_proxy, storage, zombie_factory};

#[multiversx_sc::module]
pub trait ZombieFeeding:
    storage::Storage
    + zombie_factory::ZombieFactory
{
    #[endpoint]
    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64) {
        let caller = self.blockchain().get_caller(); // Obtém o endereço de quem chamou esta função.

        // require!(caller == self.zombie_owner(&zombie_id).get(), "Only the owner of the zombie can perform this operation");
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
        // Obtém o número de dígitos que o DNA do zumbi deve ter (ex: 16).
        let dna_digits = self.dna_digits().get();
        // Calcula o valor máximo possível para um DNA com base no número de dígitos.
        // Por exemplo, se dna_digits for 16, max_dna_value será 10^16.
        let max_dna_value = u64::pow(10u64, dna_digits as u32);
        // Garante que o 'target_dna' (DNA do "alimento") não exceda o tamanho máximo permitido,
        // usando a operação de módulo (%). Isso "corta" o DNA para o número de dígitos correto.
        let verified_target_dna = target_dna % max_dna_value;
        // Calcula o DNA do novo zumbi (o "filho") como a média do DNA do zumbi original
        // e do DNA do "alimento".
        let new_dna = (my_zombie.dna + verified_target_dna) / 2;
        // Cria um novo zumbi chamando a função 'create_zombie'.
        // - 'caller': Define o proprietário do novo zumbi como o mesmo que chamou esta função.
        // - 'ManagedBuffer::from(b"NoName")': Atribui o nome "NoName" ao novo zumbi.
        // - 'new_dna': Usa o DNA recém-calculado para o novo zumbi.
        self.create_zombie(caller, ManagedBuffer::from(b"NoName"), new_dna);
    }

     #[callback]
    fn get_kitty_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<Kitty>,
        zombie_id: usize,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(kitty) => {},
            ManagedAsyncCallResult::Err(_) => {},
        }
    }

    #[endpoint]
    fn feed_on_kitty(
        &self,
        zombie_id: usize,
        kitty_id: usize,
    ) {
        let crypto_kitties_sc_address = self.crypto_kitties_sc_address().get();
        self.tx()
            .to(&crypto_kitties_sc_address)
            .typed(kitty_ownership_proxy::KittyOwnershipProxy)
            .get_kitty_by_id_endpoint(kitty_id)
            .callback(self.callbacks().get_kitty_callback(zombie_id))
            .async_call_and_exit();
    }
}