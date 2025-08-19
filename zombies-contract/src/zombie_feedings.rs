// multiversx_sc::imports!();
// multiversx_sc::derive_imports!();

// Traz as "ferramentas" e "plantas" que precisamos para o nosso jogo de zumbis.
// - `Kitty`: a planta para entender o que é um gatinho.
// - `kitty_ownership_proxy`: o manual para pedir informações sobre gatinhos a outro contrato inteligente.
// - `storage`: as regras de como guardar informações.
// - `zombie_factory`: as regras de como criar e gerenciar zumbis.
use crate::{kitty_obj::Kitty, kitty_ownership_proxy, storage, zombie_factory};

// #[multiversx_sc::module]
// Esta é a declaração do nosso contrato inteligente na blockchain. É como a "fachada" do nosso prédio de jogo.
#[multiversx_sc::module]
pub trait ZombieFeeding: // Este é o "Livro de Regras" sobre como os zumbis se alimentam e se multiplicam.
    storage::Storage // Ele precisa das regras de "onde guardar as coisas".
    + zombie_factory::ZombieFactory // E das regras de "como fabricar zumbis".
{
    // AÇÃO: ALIMENTAR E MULTIPLICAR ZUMBIS
    // #[endpoint]
    // O que faz: Esta é uma "ação especial" que um jogador pode pedir para um zumbi fazer.
    // Ela recebe o ID do zumbi, o DNA da "comida" e o tipo de "comida" (se é gatinho ou não).
    // Esta função agora é chamada de duas formas:
    // 1. Diretamente, se o alimento não for um gatinho (fluxo antigo).
    // 2. Por meio do `callback` após um gatinho ser obtido de outro contrato.
    #[endpoint]
    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64, species: ManagedBuffer) {
        let caller = self.blockchain().get_caller(); // Descobre quem está pedindo para o zumbi fazer a ação.
        require!( // REGRINHA IMPORTANTE:
            caller == self.zombie_owner(&zombie_id).get(), // Só o dono do zumbi pode dar comida a ele!
            "Only the owner of the zombie can perform this operation" // Se não for o dono, aparece essa mensagem de erro.
        );
        let my_zombie = self.zombies(&zombie_id).get(); // Pega o zumbi que vai comer.
        let dna_digits = self.dna_digits().get(); // Pega quantos números o DNA deve ter.
        let max_dna_value = u64::pow(10u64, dna_digits as u32); // Calcula o maior número que um DNA pode ser.
        let verified_target_dna = target_dna % max_dna_value; // Ajusta o DNA da comida para o tamanho certo.
        let mut new_dna = (my_zombie.dna + verified_target_dna) / 2; // Calcula o DNA do zumbi filhote (mistura dos pais).

        // NOVIDADE! REGRINHA DO GATINHO:
        // O que faz: Se a "comida" for um "gatinho" (a variável 'species' for igual a "kitty")...
        // Analogia: É como se o zumbi comesse uma "comida mágica" (o gatinho) que dá um toque especial no DNA do filhote!
        if species == ManagedBuffer::from(b"kitty") {
            // new_dna = new_dna - new_dna % 100 + 99;
            // O que faz: Esta linha muda um pouquinho o DNA do filhote.
            // Ela faz com que o DNA do filhote SEMPRE termine com os números "99".
            // Exemplo: Se o `new_dna` fosse 123456, ele vira 123499.
            // Por que existe: É uma regra de jogo para dar uma característica especial (e fácil de reconhecer)
            // aos zumbis que nascem de "gatinhos".
            new_dna = new_dna - new_dna % 100 + 99;
        }
        // self.create_zombie(caller, ManagedBuffer::from(b"NoName"), new_dna);
        // O que faz: Cria o novo zumbi filhote com o DNA especial (se comeu gatinho) ou normal.
        // O "NoName" significa que o filhote nasce sem um nome definido ainda.
        self.create_zombie(caller, ManagedBuffer::from(b"NoName"), new_dna);

        // AQUI ESTÁ UMA PARTE NOVA E MUITO IMPORTANTE!
        // Após alimentar o zumbi, ele precisa "descansar".
        // Chamamos a função `trigger_cooldown` para iniciar o tempo de espera.
        self.trigger_cooldown(zombie_id);
    }

    // AÇÃO: ATIVAR O TEMPO DE DESCANSO (COOLDOWN)
    // fn trigger_cooldown(&self, zombie_id: usize) { ... }
    // O que faz: Esta função é como apertar o botão "reiniciar o timer de descanso" do zumbi.
    // Ela recebe o ID do zumbi que precisa descansar.
    // POR QUE EXISTE: Para que, depois de uma ação (como alimentar), o zumbi precise esperar um tempo
    // antes de poder fazer outra ação importante.
    fn trigger_cooldown(&self, zombie_id: usize) {
        let cooldown_time = self.cooldown_time().get(); // Pega quanto tempo o zumbi precisa descansar (ex: 1 dia em segundos).
        self.zombies(&zombie_id).update(|my_zombie| { // Pega o zumbi certo para atualizar.
            // my_zombie.ready_time = self.blockchain().get_block_timestamp() + cooldown_time
            // O que faz: Atualiza o "relógio interno" do zumbi.
            // - `self.blockchain().get_block_timestamp()`: Pergunta ao "Relógio Mágico da Blockchain" que horas são AGORA (em segundos).
            // - `+ cooldown_time`: Soma o tempo de descanso que o zumbi precisa.
            // - **Resultado:** O `ready_time` do zumbi agora mostra uma hora no FUTURO, quando ele estará pronto novamente.
            // Analogia: É como se o brinquedo eletrônico, ao ser usado, acendesse uma luz vermelha e mostrasse no visor: "Pronto novamente em 24 horas!".
            my_zombie.ready_time = self.blockchain().get_block_timestamp() + cooldown_time
        });
    }

    // AÇÃO: VERIFICAR SE O ZUMBI ESTÁ PRONTO
    // #[view]
    // O que faz: Esta é uma "pergunta" que você pode fazer ao contrato, sem gastar nada.
    // Ela pergunta: "O zumbi X está pronto para agir AGORA?"
    // Ela recebe o ID do zumbi e retorna `true` (sim, está pronto) ou `false` (não, ainda está descansando).
    // POR QUE EXISTE: Para que o jogo possa verificar se um zumbi pode ser usado antes de tentar uma ação.
    #[view]
    fn is_ready(&self, zombie_id: usize) -> bool {
        let my_zombie = self.zombies(&zombie_id).get(); // Pega o zumbi que você quer verificar.
        // my_zombie.ready_time <= self.blockchain().get_block_timestamp()
        // O que faz: Compara o "relógio interno" do zumbi com o "Relógio Mágico da Blockchain" de AGORA.
        // - Se `ready_time` (a hora que ele estará pronto) for MENOR OU IGUAL à hora atual, significa que o tempo de descanso JÁ PASSOU.
        // - **Resultado:** Retorna `true` (sim, está pronto) se o tempo de descanso já acabou, ou `false` (não está pronto) se ainda está esperando.
        // Analogia: É como olhar para o brinquedo e ver se a luz vermelha (descansando) está apagada e a luz verde (pronto para usar) está acesa.
        my_zombie.ready_time <= self.blockchain().get_block_timestamp()
    }

    // AÇÃO: RECEBER RESPOSTA SOBRE GATINHO (CALLBACK)
    // #[callback]
    // O que faz: Esta é uma "sala de espera" para a resposta da Fazenda de Gatinhos.
    // Quando a Fazenda de Gatinhos envia o gatinho pedido, esta função é ativada.
    // POR QUE EXISTE: Para lidar com a natureza assíncrona da comunicação entre contratos.
    // A função principal (`feed_on_kitty`) não pode parar de funcionar e esperar. Ela envia o pedido e,
    // quando a resposta chegar, esta função de "retorno" é chamada pela blockchain para processá-la.
    #[callback]
    fn get_kitty_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<Kitty>, // O "envelope" que chega com o gatinho ou um erro.
        zombie_id: usize, // O ID do zumbi que estava esperando por este gatinho.
    ) {
        match result { // "Abre o envelope" para ver o que veio.
            ManagedAsyncCallResult::Ok(kitty) => { // Se o gatinho veio certinho:
                let kitty_dna = kitty.genes;
                // O que faz: Pega os genes do gatinho que chegou.
                // O tipo `Kitty` é um objeto do outro contrato (CryptoKitties).
                // `kitty.genes` acessa a propriedade 'genes' desse objeto.
                self.feed_and_multiply(zombie_id, kitty_dna, ManagedBuffer::from(b"kitty"));
                // O que faz: Chama a função para alimentar o zumbi com o DNA do gatinho.
                // O tipo da comida é "kitty", o que ativará a regra especial do DNA.
            },
            ManagedAsyncCallResult::Err(_) => {}, // Se deu algum erro ao pegar o gatinho, o código ignora. Poderia ser implementada uma lógica de erro aqui.
        }
    }

    // AÇÃO: PEDIR GATINHO PARA ALIMENTAR ZUMBI
    // #[endpoint]
    // O que faz: Esta é a ação que o jogador chama para que um zumbi se alimente de um gatinho.
    // Ela não processa o DNA diretamente. Ela inicia a comunicação com outro contrato.
    #[endpoint]
    fn feed_on_kitty(
        &self,
        zombie_id: usize, // O zumbi que vai comer.
        kitty_id: usize, // O gatinho que será comido.
    ) {
        // REGRINHA IMPORTANTE: O zumbi precisa estar pronto para agir!
        // Esta verificação é crucial para o modelo de "cooldown".
        // `require!` interrompe a execução se a condição for falsa, e a mensagem de erro é retornada.
        require!(self.is_ready(zombie_id), "Zombie is not ready yet!");

        let crypto_kitties_sc_address = self.crypto_kitties_sc_address().get(); // Pega o endereço da "Fazenda de Gatinhos".
        self.tx() // Prepara uma "carta" para enviar.
            .to(&crypto_kitties_sc_address) // O destinatário da carta é a Fazenda de Gatinhos.
            .typed(kitty_ownership_proxy::KittyOwnershipProxy) // Usa o "formulário de pedido" especial para gatinhos.
            .get_kitty_by_id_endpoint(kitty_id) // O pedido é: "Me dê o gatinho com este ID".
            .callback(self.callbacks().get_kitty_callback(zombie_id)) // Anexa o "endereço de retorno" e o ID do zumbi para o contexto.
            .async_call_and_exit(); // Envia a carta e seu contrato continua outras coisas (não espera a resposta).
            // Por que `.async_call_and_exit()`? Porque a MultiversX permite chamadas entre contratos.
            // O `async_call` envia a requisição e a transação atual termina. Quando a resposta chegar,
            // uma nova transação (interna, gerada pela blockchain) será criada para executar o `callback`.
            // Isso impede que a transação do usuário fique presa, esperando por outra.
    }

    // AÇÃO: ARMAZENAR O ENDEREÇO DO CONTRATO DE GATINHOS
    // #[storage_mapper("cryptoKittiesScAddress")]
    // O que faz: Cria uma "caixinha" segura na blockchain para guardar o endereço do contrato dos CryptoKitties.
    // Isso nos permite saber onde encontrar a "Fazenda de Gatinhos" a qualquer momento.
    #[storage_mapper("cryptoKittiesScAddress")]
    fn crypto_kitties_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}