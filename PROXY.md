# Chapter 10: What Do Zombies Eat?

It's time to feed our zombies! And what do zombies like to eat most?

Well it just so happens that CryptoZombies love to eat...

**CryptoKitties!** 😱😱😱

(Yes, I'm serious 😆 )

In order to do this we'll need to read the kittyDna from the CryptoKitties smart contract. We can do that because the CryptoKitties data is stored openly on the blockchain. Isn't the blockchain cool?!

Don't worry — our game isn't actually going to hurt anyone's CryptoKitty. We're only *reading* the CryptoKitties data, we're not able to actually delete it 😉

## Interacting with other contracts

For our contract to talk to another contract on the blockchain that we don't own, first we need to define a ***Proxy***.

Let's look at a simple example. Say there was a contract on the blockchain that looked like this:

```rust
#[multiversx_sc::contract]
pub trait Adder {

    ...

    #[endpoint]
    fn add(&self, a: BigUint, b: BigUint) -> BigUint {
        a + b
    }
}
```

This would be a simple contract where you would sum up 2 numbers and return their sum.

Now let's say we had an external contract wants to use our `add` endpoint within the `Adder` contract.

First we'd have to generate a ***Proxy*** of the `Adder` contract. To do so, in the root of the `Adder` contract we create a new file called `sc-config.toml` containing the following:

```toml
[settings]

[[proxy]]
path = "src/adder_proxy.rs"
```

Next, inside a terminal from the root of the `Adder` contract we call `sc-meta all proxy`, generating the file we defined before. After that all we need to do is to bring this new generated file inside our contract's root.

*Note: you can actually generate a copy of the proxy file inside our contract by simply specifying it's relative path:*

```toml
[settings]

[[proxy]]
path = "src/adder_proxy.rs"

[[proxy]]
path = "../../my-contract/src/adder_proxy.rs"
```

The proxy of a contract mainly contains the prototypes of the endpoints, allowing us to call them from inside another contract.

## Put it to the test

We've sketched some CryptoKitties source code for you showing how a `get_kitty` function would look like. From here we are interested of its return type which includes its "genes" (which is what our zombie game needs to form a new zombie!).

The MultiversX version of the `CryptoKitties` contract looks like this:

```rust
multiversx_sc::imports!();
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
}
```

We generated the Crypto Kitties proxy for this lesson and can be viewed in `kitty_ownership_proxy.rs`. Now that we know what the genes look like and what the `get_kitty_by_id_endpoint`, we can prepare ourselves to use the proxy, by making sure our contract knows these elements.

### Task

1. Import the `Kitty`, `KittyGenes` and `Color` in `kitty_obj.rs`