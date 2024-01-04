use scrypto::prelude::*;

#[blueprint]
mod poke_dix {
    struct PokeDix {
        
    }

    impl PokeDix {
        pub fn instantiate_hello() -> Global<PokeDix> {
            Self {

            }.instantiate()
            .prepare_to_globalize(
                OwnerRole::None
            )
            .globalize()
        }

    }
}
