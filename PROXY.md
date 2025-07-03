<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CryptoZombies Capítulo 10 - Guia Bilíngue</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
            line-height: 1.6;
        }

        .container {
            max-width: 1400px;
            margin: 0 auto;
            background: rgba(255, 255, 255, 0.95);
            border-radius: 20px;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }

        .header {
            background: linear-gradient(135deg, #2c3e50 0%, #3498db 100%);
            color: white;
            text-align: center;
            padding: 40px 20px;
            position: relative;
            overflow: hidden;
        }

        .header::before {
            content: '';
            position: absolute;
            top: -50%;
            left: -50%;
            width: 200%;
            height: 200%;
            background: radial-gradient(circle, rgba(255,255,255,0.1) 0%, transparent 70%);
            animation: pulse 4s ease-in-out infinite;
        }

        @keyframes pulse {
            0%, 100% { transform: scale(1); opacity: 0.1; }
            50% { transform: scale(1.05); opacity: 0.2; }
        }

        .header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
            position: relative;
            z-index: 2;
        }

        .header p {
            font-size: 1.2em;
            opacity: 0.9;
            position: relative;
            z-index: 2;
        }

        .language-toggle {
            display: flex;
            justify-content: center;
            padding: 20px;
            background: #f8f9fa;
            border-bottom: 1px solid #dee2e6;
        }

        .toggle-button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 12px 30px;
            margin: 0 10px;
            border-radius: 25px;
            cursor: pointer;
            font-size: 16px;
            font-weight: 600;
            transition: all 0.3s ease;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
        }

        .toggle-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
        }

        .toggle-button.active {
            background: linear-gradient(135deg, #2c3e50 0%, #3498db 100%);
            transform: translateY(-1px);
        }

        .content {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 0;
            min-height: 600px;
        }

        .language-column {
            padding: 40px;
            transition: all 0.3s ease;
        }

        .language-column.english {
            background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
            border-right: 2px solid #dee2e6;
        }

        .language-column.portuguese {
            background: linear-gradient(135deg, #e8f5e8 0%, #d4edda 100%);
        }

        .language-column h2 {
            color: #2c3e50;
            margin-bottom: 30px;
            font-size: 2em;
            position: relative;
            padding-bottom: 10px;
        }

        .language-column h2::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 0;
            width: 60px;
            height: 3px;
            background: linear-gradient(90deg, #667eea, #764ba2);
            border-radius: 2px;
        }

        .language-column h3 {
            color: #3498db;
            margin: 25px 0 15px 0;
            font-size: 1.5em;
        }

        .language-column h4 {
            color: #2c3e50;
            margin: 20px 0 10px 0;
            font-size: 1.2em;
        }

        .language-column p {
            margin-bottom: 15px;
            color: #333;
            text-align: justify;
        }

        .language-column code {
            background: #f1f3f4;
            padding: 2px 6px;
            border-radius: 4px;
            font-family: 'Courier New', monospace;
            color: #d63384;
        }

        .language-column pre {
            background: #2d3748;
            color: #e2e8f0;
            padding: 20px;
            border-radius: 10px;
            overflow-x: auto;
            margin: 20px 0;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        }

        .language-column pre code {
            background: none;
            color: #e2e8f0;
            padding: 0;
        }

        .task-box {
            background: linear-gradient(135deg, #ffeaa7 0%, #fab1a0 100%);
            border: 1px solid #fdcb6e;
            border-radius: 10px;
            padding: 20px;
            margin: 20px 0;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        }

        .task-box h4 {
            color: #2d3436;
            margin-bottom: 10px;
        }

        .emoji {
            font-size: 1.2em;
        }

        .highlight {
            background: linear-gradient(135deg, #74b9ff 0%, #0984e3 100%);
            color: white;
            padding: 2px 6px;
            border-radius: 4px;
            font-weight: 600;
        }

        .mobile-toggle {
            display: none;
        }

        @media (max-width: 768px) {
            .content {
                grid-template-columns: 1fr;
            }
            
            .language-column.english {
                border-right: none;
                border-bottom: 2px solid #dee2e6;
            }
            
            .mobile-toggle {
                display: block;
            }
            
            .language-column {
                padding: 20px;
            }
            
            .header h1 {
                font-size: 2em;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>CryptoZombies Capítulo 10</h1>
            <p>O que os Zumbis Comem? | What Do Zombies Eat?</p>
        </div>

        <div class="language-toggle">
            <button class="toggle-button active" onclick="showBoth()">Ambos Idiomas</button>
            <button class="toggle-button" onclick="showEnglish()">English Only</button>
            <button class="toggle-button" onclick="showPortuguese()">Português Apenas</button>
        </div>

        <div class="content" id="content">
            <div class="language-column english" id="english-column">
                <h2>🇺🇸 English Version</h2>
                
                <p>It's time to feed our zombies! And what do zombies like to eat most?</p>
                
                <p>Well it just so happens that CryptoZombies love to eat...</p>
                
                <p><strong>CryptoKitties!</strong> <span class="emoji">😱😱😱</span></p>
                
                <p>(Yes, I'm serious <span class="emoji">😆</span>)</p>
                
                <p>In order to do this we'll need to read the kittyDna from the CryptoKitties smart contract. We can do that because the CryptoKitties data is stored openly on the blockchain. Isn't the blockchain cool?!</p>
                
                <p>Don't worry — our game isn't actually going to hurt anyone's CryptoKitty. We're only <em>reading</em> the CryptoKitties data, we're not able to actually delete it <span class="emoji">😉</span></p>

                <h3>Interacting with other contracts</h3>
                
                <p>For our contract to talk to another contract on the blockchain that we don't own, first we need to define a <span class="highlight">Proxy</span>.</p>
                
                <p>Let's look at a simple example. Say there was a contract on the blockchain that looked like this:</p>

                <pre><code>#[multiversx_sc::contract]
pub trait Adder {

    ...

    #[endpoint]
    fn add(&self, a: BigUint, b: BigUint) -> BigUint {
        a + b
    }
}</code></pre>

                <p>This would be a simple contract where you would sum up 2 numbers and return their sum.</p>
                
                <p>Now let's say we had an external contract wants to use our <code>add</code> endpoint within the <code>Adder</code> contract.</p>
                
                <p>First we'd have to generate a <span class="highlight">Proxy</span> of the <code>Adder</code> contract. To do so, in the root of the <code>Adder</code> contract we create a new file called <code>sc-config.toml</code> containing the following:</p>

                <pre><code>[settings]

[[proxy]]
path = "src/adder_proxy.rs"</code></pre>

                <p>Next, inside a terminal from the root of the <code>Adder</code> contract we call <code>sc-meta all proxy</code>, generating the file we defined before. After that all we need to do is to bring this new generated file inside our contract's root.</p>
                
                <p><em>Note: you can actually generate a copy of the proxy file inside our contract by simply specifying it's relative path:</em></p>

                <pre><code>[settings]

[[proxy]]
path = "src/adder_proxy.rs"

[[proxy]]
path = "../../my-contract/src/adder_proxy.rs"</code></pre>

                <p>The proxy of a contract mainly contains the prototypes of the endpoints, allowing us to call them from inside another contract.</p>

                <h3>Put it to the test</h3>
                
                <p>We've sketched some CryptoKitties source code for you showing how a <code>get_kitty</code> function would look like. From here we are interested of its return type which includes its "genes" (which is what our zombie game needs to form a new zombie!).</p>
                
                <p>The MultiversX version of the <code>CryptoKitties</code> contract looks like this:</p>

                <pre><code>multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Kitty {
    pub genes: KittyGenes,
    pub birth_time: u64,   // timestamp
    pub cooldown_end: u64, // timestamp, used for pregnancy timer and siring cooldown
    pub matron_id: u32,
    pub sire_id: u32,
    pub siring_with_id: u32, // for pregnant cats, 0 otherwise
    pub nr_children: u16,    // cooldown period increases exponentially with every breeding/siring
    pub generation: u16,     // max(sire_gen, matron_gen) + 1. Generation also influences cooldown.
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct KittyGenes {
    pub fur_color: Color,
    pub eye_color: Color,
    pub meow_power: u8, // the higher the value, the louder the cat
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[multiversx_sc::contract]
pub trait KittyOwnership {

  ...

    #[view(getKittyById)]
    fn get_kitty_by_id_endpoint(&self, kitty_id: u32) -> Kitty {
        ...
    }
}</code></pre>

                <p>We generated the Crypto Kitties proxy for this lesson and can be viewed in <code>kitty_ownership_proxy.rs</code>. Now that we know what the genes look like and what the <code>get_kitty_by_id_endpoint</code>, we can prepare ourselves to use the proxy, by making sure our contract knows these elements.</p>

                <div class="task-box">
                    <h4>Task</h4>
                    <p>1. Import the <code>Kitty</code>, <code>KittyGenes</code> and <code>Color</code> in <code>kitty_obj.rs</code></p>
                </div>
            </div>

            <div class="language-column portuguese" id="portuguese-column">
                <h2>🇧🇷 Versão em Português</h2>
                
                <p>É hora de alimentar nossos zumbis! E o que os zumbis mais gostam de comer?</p>
                
                <p>Bem, acontece que os CryptoZombies adoram comer...</p>
                
                <p><strong>CryptoKitties!</strong> <span class="emoji">😱😱😱</span></p>
                
                <p>(Sim, estou falando sério <span class="emoji">😆</span>)</p>
                
                <p>Para fazer isso, precisaremos ler o kittyDna do contrato inteligente CryptoKitties. Podemos fazer isso porque os dados CryptoKitties são armazenados abertamente no blockchain. O blockchain não é incrível?!</p>
                
                <p>Não se preocupe — nosso jogo não vai realmente machucar nenhum CryptoKitty. Estamos apenas <em>lendo</em> os dados CryptoKitties, não somos capazes de realmente deletá-los <span class="emoji">😉</span></p>

                <h3>Interagindo com outros contratos</h3>
                
                <p>Para nosso contrato conversar com outro contrato no blockchain que não possuímos, primeiro precisamos definir um <span class="highlight">Proxy</span>.</p>
                
                <p>Vamos ver um exemplo simples. Digamos que houvesse um contrato no blockchain que se parecesse com isso:</p>

                <pre><code>#[multiversx_sc::contract]
pub trait Adder {

    ...

    #[endpoint]
    fn add(&self, a: BigUint, b: BigUint) -> BigUint {
        a + b
    }
}</code></pre>

                <p>Este seria um contrato simples onde você somaria 2 números e retornaria sua soma.</p>
                
                <p>Agora vamos dizer que tínhamos um contrato externo que quer usar nosso endpoint <code>add</code> dentro do contrato <code>Adder</code>.</p>
                
                <p>Primeiro teríamos que gerar um <span class="highlight">Proxy</span> do contrato <code>Adder</code>. Para fazer isso, na raiz do contrato <code>Adder</code> criamos um novo arquivo chamado <code>sc-config.toml</code> contendo o seguinte:</p>

                <pre><code>[settings]

[[proxy]]
path = "src/adder_proxy.rs"</code></pre>

                <p>Em seguida, dentro de um terminal da raiz do contrato <code>Adder</code> chamamos <code>sc-meta all proxy</code>, gerando o arquivo que definimos antes. Depois disso, tudo que precisamos fazer é trazer este novo arquivo gerado para dentro da raiz do nosso contrato.</p>
                
                <p><em>Nota: você pode realmente gerar uma cópia do arquivo proxy dentro do nosso contrato simplesmente especificando seu caminho relativo:</em></p>

                <pre><code>[settings]

[[proxy]]
path = "src/adder_proxy.rs"

[[proxy]]
path = "../../my-contract/src/adder_proxy.rs"</code></pre>

                <p>O proxy de um contrato principalmente contém os protótipos dos endpoints, nos permitindo chamá-los de dentro de outro contrato.</p>

                <h3>Coloque à prova</h3>
                
                <p>Esboçamos algum código fonte CryptoKitties para você mostrando como uma função <code>get_kitty</code> se pareceria. A partir daqui estamos interessados em seu tipo de retorno que inclui seus "genes" (que é o que nosso jogo de zumbis precisa para formar um novo zumbi!).</p>
                
                <p>A versão MultiversX do contrato <code>CryptoKitties</code> se parece com isso:</p>

                <pre><code>multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Kitty {
    pub genes: KittyGenes,
    pub birth_time: u64,   // timestamp
    pub cooldown_end: u64, // timestamp, usado para timer de gravidez e cooldown de acasalamento
    pub matron_id: u32,
    pub sire_id: u32,
    pub siring_with_id: u32, // para gatos grávidos, 0 caso contrário
    pub nr_children: u16,    // período de cooldown aumenta exponencialmente com cada reprodução/acasalamento
    pub generation: u16,     // max(sire_gen, matron_gen) + 1. Geração também influencia cooldown.
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct KittyGenes {
    pub fur_color: Color,
    pub eye_color: Color,
    pub meow_power: u8, // quanto maior o valor, mais alto o gato
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[multiversx_sc::contract]
pub trait KittyOwnership {

  ...

    #[view(getKittyById)]
    fn get_kitty_by_id_endpoint(&self, kitty_id: u32) -> Kitty {
        ...
    }
}</code></pre>

                <p>Geramos o proxy Crypto Kitties para esta lição e pode ser visualizado em <code>kitty_ownership_proxy.rs</code>. Agora que sabemos como os genes se parecem e o que é o <code>get_kitty_by_id_endpoint</code>, podemos nos preparar para usar o proxy, garantindo que nosso contrato conhece esses elementos.</p>

                <div class="task-box">
                    <h4>Tarefa</h4>
                    <p>1. Importe o <code>Kitty</code>, <code>KittyGenes</code> e <code>Color</code> em <code>kitty_obj.rs</code></p>
                </div>
            </div>
        </div>
    </div>

    <script>
        function showBoth() {
            const content = document.getElementById('content');
            const englishColumn = document.getElementById('english-column');
            const portugueseColumn = document.getElementById('portuguese-column');
            
            content.style.gridTemplateColumns = '1fr 1fr';
            englishColumn.style.display = 'block';
            portugueseColumn.style.display = 'block';
            
            // Update active button
            document.querySelectorAll('.toggle-button').forEach(btn => btn.classList.remove('active'));
            event.target.classList.add('active');
        }

        function showEnglish() {
            const content = document.getElementById('content');
            const englishColumn = document.getElementById('english-column');
            const portugueseColumn = document.getElementById('portuguese-column');
            
            content.style.gridTemplateColumns = '1fr';
            englishColumn.style.display = 'block';
            portugueseColumn.style.display = 'none';
            
            // Update active button
            document.querySelectorAll('.toggle-button').forEach(btn => btn.classList.remove('active'));
            event.target.classList.add('active');
        }

        function showPortuguese() {
            const content = document.getElementById('content');
            const englishColumn = document.getElementById('english-column');
            const portugueseColumn = document.getElementById('portuguese-column');
            
            content.style.gridTemplateColumns = '1fr';
            englishColumn.style.display = 'none';
            portugueseColumn.style.display = 'block';
            
            // Update active button
            document.querySelectorAll('.toggle-button').forEach(btn => btn.classList.remove('active'));
            event.target.classList.add('active');
        }
    </script>
</body>
</html>