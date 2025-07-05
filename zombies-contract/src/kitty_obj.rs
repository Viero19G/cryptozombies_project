use multiversx_sc::derive_imports::*; 

#[type_abi] // Atributo que informa ao compilador para gerar a Interface Binária de Aplicação (ABI) para esta struct.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)] // Deriva traits que permitem à struct ser convertida de/para bytes para armazenamento na blockchain ou comunicação.
pub struct Kitty {
    pub genes: KittyGenes, // Campo 'genes': Armazena as características genéticas do gatinho, usando a struct `KittyGenes`.
    pub birth_time: u64,   // Campo 'birth_time': O tempo de nascimento do gatinho, em formato de timestamp (u64).
    pub cooldown_end: u64, // Campo 'cooldown_end': O tempo em que o gatinho estará pronto para uma nova ação (ex: acasalar), também em timestamp (u64).
    pub matron_id: u32,    // Campo 'matron_id': O ID da mãe do gatinho (u32).
    pub sire_id: u32,      // Campo 'sire_id': O ID do pai do gatinho (u32).
    pub siring_with_id: u32, // Campo 'siring_with_id': O ID do gatinho com o qual este está acasalando atualmente (u32).
    pub nr_children: u16,  // Campo 'nr_children': O número de filhos que este gatinho já teve (u16).
    pub generation: u16,   // Campo 'generation': A geração do gatinho (ex: 0 para os primeiros, 1 para seus filhos, etc.) (u16).
}

#[type_abi] // Gera ABI para `KittyGenes`.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)] // Permite serialização/deserialização de `KittyGenes`.
pub struct KittyGenes {
    pub fur_color: Color,    // Campo 'fur_color': A cor da pelagem do gatinho, usando a struct `Color`.
    pub eye_color: Color,    // Campo 'eye_color': A cor dos olhos do gatinho, usando a struct `Color`.
    pub meow_power: u8,      // Campo 'meow_power': O "poder do miado" do gatinho, um valor de 0 a 255 (u8).
}

#[type_abi] // Gera ABI para `Color`.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)] // Permite serialização/deserialização de `Color`.
pub struct Color {
    pub r: u8, // Campo 'r': Componente vermelho da cor (0-255).
    pub g: u8, // Campo 'g': Componente verde da cor (0-255).
    pub b: u8, // Campo 'b': Componente azul da cor (0-255).
}

// Implementação de métodos para a struct `Color`
impl Color {
    // pub fn as_u64(&self) -> u64 { ... }
    // O que faz: Esta função tenta combinar os componentes de cor (r, g, b) em um único número `u64`.
    // POR QUE É ESCRITA ASSIM: A intenção é empacotar os 3 bytes de cor em 24 bits de um `u64`.
    //
    // ANALOGIA DO BAÚ E DAS CAIXAS (para `Color::as_u64`):
    // Imagine um baú de 64 compartimentos (bits). Cada `u8` é uma caixa de 8 compartimentos.
    // Queremos empilhar R, G e B, um ao lado do outro, sem sobreposição.
    //
    // A LÓGICA NO SEU CÓDIGO: `(self.r.to_be() << 8 | self.r.to_be()) << 8 | self.r.to_be()`
    //
    // ATENÇÃO: Há um pequeno detalhe aqui. No seu código atual, você está usando `self.r.to_be()`
    // para todos os três componentes (r, g, b). Isso significa que, em vez de combinar as cores
    // vermelho, verde e azul, você estará combinando o valor do componente VERMELHO três vezes.
    //
    // O QUE SEU CÓDIGO FAZ AGORA:
    // 1. `self.r.to_be() << 8`: Pega o valor de `r` e o empurra 8 bits para a esquerda.
    //    Visualização: `[R][00000000]`
    // 2. `| self.r.to_be()`: Encaixa *novamente* o valor de `r` nos 8 bits mais à direita.
    //    Visualização: `[R][R]` (o valor de R é duplicado)
    // 3. `(...) << 8`: Pega o resultado `[R][R]` (16 bits) e o empurra mais 8 bits para a esquerda.
    //    Visualização: `[R][R][00000000]`
    // 4. `| self.r.to_be()`: Encaixa *pela terceira vez* o valor de `r` nos 8 bits mais à direita.
    //    Visualização: `[R][R][R]` (o valor de R é triplicado no `u64`)
    //
    // PARA CORRIGIR E OBTER `[R][G][B]`, a linha deveria ser:
    // `((self.r.to_be() as u64) << 16) | ((self.g.to_be() as u64) << 8) | (self.b.to_be() as u64)`
    // OU, no seu estilo de encadeamento (que também é válido):
    // `( (self.r.to_be() as u64) << 8 | (self.g.to_be() as u64) ) << 8 | (self.b.to_be() as u64)`
    //
    // O `.to_be()`: Garante que os bytes sejam organizados em "Big Endian" (o byte mais significativo primeiro),
    // o que é vital para consistência em blockchain. `as u64` converte o `u8` para `u64` para permitir o deslocamento.
    pub fn as_u64(&self) -> u64 {
        ((self.r.to_be() as u64) << 8 | self.r.to_be() as u64) << 8 | self.r.to_be() as u64
    }
}

// Implementação de métodos para a struct `KittyGenes`
impl KittyGenes {
    // pub fn get_as_u64(&self) -> u64 { ... }
    // O que faz: Esta função combina os três elementos de `KittyGenes` (fur_color, eye_color, meow_power)
    // em um único número `u64`.
    // POR QUE É ESCRITA ASSIM: O objetivo é empacotar os bits de cada componente em um único `u64`
    // sem que eles se sobreponham, na ordem `[fur_color][eye_color][meow_power]`.
    //
    // ANALOGIA DO BAÚ E DAS CAIXAS (para `KittyGenes::get_as_u64`):
    // Agora, cada `Color::as_u64()` é uma "caixa de 24 compartimentos" (bits). `meow_power` é uma "caixa de 8 compartimentos".
    //
    // 1. Combine `fur_color` e `eye_color` primeiro:
    //    - `self.fur_color.as_u64() << 24`: Pega a caixa de 24 bits da `fur_color` e a **empurra 24 compartimentos para a esquerda**.
    //      POR QUE `<< 24`? Porque `fur_color` é seguido por `eye_color`, e `eye_color` (como uma `Color`) ocupa **24 bits**.
    //      Precisamos abrir exatamente 24 bits de espaço para `eye_color` à direita de `fur_color`.
    //      Visualização: `[...000000000000000000000000][FUR_COLOR][000000000000000000000000]`
    //    - `| self.eye_color.as_u64()`: Encaixa a caixa de 24 bits da `eye_color` nos 24 espaços vazios que acabamos de abrir.
    //      Resultado parcial: `[...0000000000000000][FUR_COLOR][EYE_COLOR]` (agora 48 bits ocupados no `u64`)
    //
    // 2. Desloque o resultado combinado `[FUR_COLOR][EYE_COLOR]` e adicione `meow_power`:
    //    - `(...) << 8`: Pega o bloco combinado `[FUR_COLOR][EYE_COLOR]` (que tem 48 bits) e o **empurra 8 compartimentos para a esquerda**.
    //      POR QUE `<< 8`? Porque este bloco combinado é seguido por `meow_power`, e `meow_power` (como um `u8`) ocupa **8 bits**.
    //      Precisamos abrir exatamente 8 bits de espaço para `meow_power` à direita do bloco combinado.
    //      Visualização: `[...00000000][FUR_COLOR][EYE_COLOR][00000000]`
    //    - `| self.meow_power.to_be() as u64`: Encaixa a caixa de 8 bits do `meow_power` nos 8 espaços vazios que acabamos de abrir.
    //      Resultado final: `[...00000000][FUR_COLOR][EYE_COLOR][MEOW_POWER]` (total de 56 bits ocupados no `u64`)
    //
    // O `as u64` em `meow_power.to_be() as u64` é necessário para garantir que o tipo seja `u64` antes da operação OR.
    pub fn get_as_u64(&self) -> u64 {
        ((self.fur_color.as_u64() << 24 | self.eye_color.as_u64()) << 8
            | self.meow_power.to_be() as u64)
    }
}
