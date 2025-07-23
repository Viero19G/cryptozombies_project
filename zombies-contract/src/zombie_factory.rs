multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{storage, zombie::Zombie}; // Importa o módulo de armazenamento e a struct Zombie

#[multiversx_sc::module]
pub trait ZombieFactory: storage::Storage {
    // A função principal para "dar à luz" um novo zumbi
    fn create_zombie(&self, owner: ManagedAddress, name: ManagedBuffer, dna: u64) {
        self.zombie_last_index().update(|id| { // Pega o próximo ID disponível para o zumbi
            self.new_zombie_event(*id, &name, dna); // Emite um evento para avisar que um novo zumbi nasceu

            // LINHA CHAVE 1: Obtém o tempo de recarga (cooldown)
            let cooldown_time = self.cooldown_time().get(); // Pega o valor do "tempo de espera" do contrato (ex: 86400 segundos)

            // LINHA CHAVE 2: Cria o zumbi com nível e tempo de prontidão
            self.zombies(id).set(Zombie {
                name, // O nome do zumbi
                dna, // O DNA do zumbi
                level: 1u16, // Nível inicial do zumbi: sempre 1 quando nasce
                ready_time: self.blockchain().get_block_timestamp() + cooldown_time, // QUANDO o zumbi estará pronto para agir novamente
            });
            self.owned_zombies(&owner).insert(*id); // Registra que o 'owner' é dono deste zumbi
            self.zombie_owner(id).set(owner); // Registra quem é o proprietário deste zumbi
            *id += 1; // Incrementa o contador para o próximo zumbi
        });
    }

    // Função para gerar um DNA aleatório (já explicamos antes)
    #[view]
    fn generate_random_dna(&self) -> u64 {
        let mut rand_source = RandomnessSource::new();
        let dna_digits = self.dna_digits().get();
        let max_dna_value = u64::pow(10u64, dna_digits as u32);
        rand_source.next_u64_in_range(0u64, max_dna_value)
    }

    // Endpoint para criar um zumbi aleatório (chamado por um usuário)
    #[endpoint]
    fn create_random_zombie(&self, name: ManagedBuffer) {
        let caller = self.blockchain().get_caller(); // Quem chamou esta função
        require!(
            self.owned_zombies(&caller).is_empty(), // Verifica se o chamador JÁ tem um zumbi
            "You already own a zombie" // Se tiver, impede a criação e mostra esta mensagem
        );
        let rand_dna = self.generate_random_dna(); // Gera um DNA aleatório
        self.create_zombie(caller, name, rand_dna); // Chama a função principal 'create_zombie' para criar o zumbi
    }

    // Evento para notificar a criação de um novo zumbi
    #[event("newZombieEvent")]
    fn new_zombie_event(
        &self,
        #[indexed] zombie_id: usize,
        name: &ManagedBuffer,
        #[indexed] dna: u64,
    );
}