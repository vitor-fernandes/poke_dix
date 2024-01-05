use scrypto::prelude::*;

#[derive(ScryptoSbor, Debug, Clone)]
enum Type {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Fairy,
    Exploit
}

#[derive(ScryptoSbor, NonFungibleData)]
struct Pokemon {
    number: u64,
    name: String,
    typ: Type
}  

#[blueprint]
mod poke_dix {
    struct PokeDix {
        vault_nft_pokemon: NonFungibleVault,
        resource_manager_nft_pokemon: ResourceManager,

        collected_xrd: FungibleVault,

        mint_price: Decimal,

        all_pokemons: HashMap<u8, Pokemon>,
        minted_pokemons: u64
    }

    impl PokeDix {
        
        pub fn instantiate_hello() -> Global<PokeDix> {
            let tmp_rm_nft_pokemon: ResourceManager = ResourceBuilder::new_integer_non_fungible::<Pokemon>(OwnerRole::None)
            .metadata(metadata!(
                init {
                    "name" => "Pokémon Radix", locked;
                    "symbol" => "rPKM", locked;
                    "description" => "An NFT collection based on Pokémon", locked;
                }
            ))
            .mint_roles(mint_roles! {
                minter => rule!(allow_all);
                minter_updater => rule!(deny_all);
            })
            .create_with_no_initial_supply();
            
            Self {
                vault_nft_pokemon: NonFungibleVault::with_bucket(NonFungibleBucket::new(tmp_rm_nft_pokemon.address())),
                resource_manager_nft_pokemon: tmp_rm_nft_pokemon,
                collected_xrd: FungibleVault::new(XRD),
                mint_price: dec!("10"),
                minted_pokemons: 0,
                all_pokemons: HashMap::new()
            }.instantiate()
            .prepare_to_globalize(
                OwnerRole::None
            )
            .globalize()
        }

        pub fn mint(&mut self) -> NonFungibleBucket {
            let mut random_number = Runtime::generate_ruid()[0];
            
            if random_number > 151 {
                random_number = random_number - 151;
            };

            let tmp_poke_data: &Pokemon = self.all_pokemons.get(&random_number).unwrap();

            let tmp_pokemon_bucket = self.resource_manager_nft_pokemon.mint_non_fungible(
                &NonFungibleLocalId::integer(self.minted_pokemons),
                Pokemon {
                    number: tmp_poke_data.number.clone(),
                    name: tmp_poke_data.name.clone(),
                    typ: tmp_poke_data.typ.clone()
                },
            ).as_non_fungible();

            self.minted_pokemons += 1;

            return tmp_pokemon_bucket;
        }

        pub fn get_nft_data(&self, tmp_pokemon: Bucket) -> Bucket {
            let nft_local_id = tmp_pokemon.as_non_fungible().non_fungible_local_id();
            info!("LocalId: {:?}", nft_local_id);
            let nft_data: Pokemon = tmp_pokemon.as_non_fungible().non_fungible().data();
            info!("Number: {}", nft_data.number);
            info!("Name: {}", nft_data.name);
            info!("Type: {:?}", nft_data.typ);

            return tmp_pokemon;
        }

        pub fn get_pokemon(&self, _index: u8){
            info!("Pokemon Number: {}", &self.all_pokemons[&_index].number);
            info!("Pokemon Name: {}", &self.all_pokemons[&_index].name);
            info!("Pokemon Type: {:?}", &self.all_pokemons[&_index].typ);
        }

        pub fn register_all_pokemons(&mut self) {
            assert_eq!(self.all_pokemons.len(), 0, "Already Registered");
            let mut tmp_pokemons: HashMap<u8, Pokemon> = HashMap::new();
            tmp_pokemons.insert(0, Pokemon {number: 0, name: "XPL".into(), typ: Type::Exploit});
            tmp_pokemons.insert(1, Pokemon {number: 1, name: "Bulbasaur".into(), typ: Type::Grass});
            tmp_pokemons.insert(2, Pokemon {number: 2, name: "Ivysaur".into(), typ: Type::Grass});
            tmp_pokemons.insert(3, Pokemon {number: 3, name: "Venusaur".into(), typ: Type::Grass});
            tmp_pokemons.insert(4, Pokemon {number: 4, name: "Charmander".into(), typ: Type::Fire});
            tmp_pokemons.insert(5, Pokemon {number: 5, name: "Charmeleon".into(), typ: Type::Fire});
            tmp_pokemons.insert(6, Pokemon {number: 6, name: "Charizard".into(), typ: Type::Fire});
            tmp_pokemons.insert(7, Pokemon {number: 7, name: "Squirtle".into(), typ: Type::Water});
            tmp_pokemons.insert(8, Pokemon {number: 8, name: "Wartortle".into(), typ: Type::Water});
            tmp_pokemons.insert(9, Pokemon {number: 9, name: "Blastoise".into(), typ: Type::Water});
            tmp_pokemons.insert(10, Pokemon {number: 10, name: "Caterpie".into(), typ: Type::Bug});
            tmp_pokemons.insert(11, Pokemon {number: 11, name: "Metapod".into(), typ: Type::Bug});
            tmp_pokemons.insert(12, Pokemon {number: 12, name: "Butterfree".into(), typ: Type::Bug});
            tmp_pokemons.insert(13, Pokemon {number: 13, name: "Weedle".into(), typ: Type::Bug});
            tmp_pokemons.insert(14, Pokemon {number: 14, name: "Kakuna".into(), typ: Type::Bug});
            tmp_pokemons.insert(15, Pokemon {number: 15, name: "Beedrill".into(), typ: Type::Bug});
            tmp_pokemons.insert(16, Pokemon {number: 16, name: "Pidgey".into(), typ: Type::Normal});
            tmp_pokemons.insert(17, Pokemon {number: 17, name: "Pidgeotto".into(), typ: Type::Normal});
            tmp_pokemons.insert(18, Pokemon {number: 18, name: "Pidgeot".into(), typ: Type::Normal});
            tmp_pokemons.insert(19, Pokemon {number: 19, name: "Rattata".into(), typ: Type::Normal});
            tmp_pokemons.insert(20, Pokemon {number: 20, name: "Raticate".into(), typ: Type::Normal});
            tmp_pokemons.insert(21, Pokemon {number: 21, name: "Spearow".into(), typ: Type::Normal});
            tmp_pokemons.insert(22, Pokemon {number: 22, name: "Fearow".into(), typ: Type::Normal});
            tmp_pokemons.insert(23, Pokemon {number: 23, name: "Ekans".into(), typ: Type::Poison});
            tmp_pokemons.insert(24, Pokemon {number: 24, name: "Arbok".into(), typ: Type::Poison});
            tmp_pokemons.insert(25, Pokemon {number: 25, name: "Pikachu".into(), typ: Type::Electric});
            tmp_pokemons.insert(26, Pokemon {number: 26, name: "Raichu".into(), typ: Type::Electric});
            tmp_pokemons.insert(27, Pokemon {number: 27, name: "Sandshrew".into(), typ: Type::Ground});
            tmp_pokemons.insert(28, Pokemon {number: 28, name: "Sandslash".into(), typ: Type::Ground});
            tmp_pokemons.insert(29, Pokemon {number: 29, name: "Nidoran-f".into(), typ: Type::Poison});
            tmp_pokemons.insert(30, Pokemon {number: 30, name: "Nidorina".into(), typ: Type::Poison});
            tmp_pokemons.insert(31, Pokemon {number: 31, name: "Nidoqueen".into(), typ: Type::Poison});
            tmp_pokemons.insert(32, Pokemon {number: 32, name: "Nidoran-m".into(), typ: Type::Poison});
            tmp_pokemons.insert(33, Pokemon {number: 33, name: "Nidorino".into(), typ: Type::Poison});
            tmp_pokemons.insert(34, Pokemon {number: 34, name: "Nidoking".into(), typ: Type::Poison});
            tmp_pokemons.insert(35, Pokemon {number: 35, name: "Clefairy".into(), typ: Type::Fairy});
            tmp_pokemons.insert(36, Pokemon {number: 36, name: "Clefable".into(), typ: Type::Fairy});
            tmp_pokemons.insert(37, Pokemon {number: 37, name: "Vulpix".into(), typ: Type::Fire});
            tmp_pokemons.insert(38, Pokemon {number: 38, name: "Ninetales".into(), typ: Type::Fire});
            tmp_pokemons.insert(39, Pokemon {number: 39, name: "Jigglypuff".into(), typ: Type::Normal});
            tmp_pokemons.insert(40, Pokemon {number: 40, name: "Wigglytuff".into(), typ: Type::Normal});
            tmp_pokemons.insert(41, Pokemon {number: 41, name: "Zubat".into(), typ: Type::Poison});
            tmp_pokemons.insert(42, Pokemon {number: 42, name: "Golbat".into(), typ: Type::Poison});
            tmp_pokemons.insert(43, Pokemon {number: 43, name: "Oddish".into(), typ: Type::Grass});
            tmp_pokemons.insert(44, Pokemon {number: 44, name: "Gloom".into(), typ: Type::Grass});
            tmp_pokemons.insert(45, Pokemon {number: 45, name: "Vileplume".into(), typ: Type::Grass});
            tmp_pokemons.insert(46, Pokemon {number: 46, name: "Paras".into(), typ: Type::Bug});
            tmp_pokemons.insert(47, Pokemon {number: 47, name: "Parasect".into(), typ: Type::Bug});
            tmp_pokemons.insert(48, Pokemon {number: 48, name: "Venonat".into(), typ: Type::Bug});
            tmp_pokemons.insert(49, Pokemon {number: 49, name: "Venomoth".into(), typ: Type::Bug});
            tmp_pokemons.insert(50, Pokemon {number: 50, name: "Diglett".into(), typ: Type::Ground});
            tmp_pokemons.insert(51, Pokemon {number: 51, name: "Dugtrio".into(), typ: Type::Ground});
            tmp_pokemons.insert(52, Pokemon {number: 52, name: "Meowth".into(), typ: Type::Normal});
            tmp_pokemons.insert(53, Pokemon {number: 53, name: "Persian".into(), typ: Type::Normal});
            tmp_pokemons.insert(54, Pokemon {number: 54, name: "Psyduck".into(), typ: Type::Water});
            tmp_pokemons.insert(55, Pokemon {number: 55, name: "Golduck".into(), typ: Type::Water});
            tmp_pokemons.insert(56, Pokemon {number: 56, name: "Mankey".into(), typ: Type::Fighting});
            tmp_pokemons.insert(57, Pokemon {number: 57, name: "Primeape".into(), typ: Type::Fighting});
            tmp_pokemons.insert(58, Pokemon {number: 58, name: "Growlithe".into(), typ: Type::Fire});
            tmp_pokemons.insert(59, Pokemon {number: 59, name: "Arcanine".into(), typ: Type::Fire});
            tmp_pokemons.insert(60, Pokemon {number: 60, name: "Poliwag".into(), typ: Type::Water});
            tmp_pokemons.insert(61, Pokemon {number: 61, name: "Poliwhirl".into(), typ: Type::Water});
            tmp_pokemons.insert(62, Pokemon {number: 62, name: "Poliwrath".into(), typ: Type::Water});
            tmp_pokemons.insert(63, Pokemon {number: 63, name: "Abra".into(), typ: Type::Psychic});
            tmp_pokemons.insert(64, Pokemon {number: 64, name: "Kadabra".into(), typ: Type::Psychic});
            tmp_pokemons.insert(65, Pokemon {number: 65, name: "Alakazam".into(), typ: Type::Psychic});
            tmp_pokemons.insert(66, Pokemon {number: 66, name: "Machop".into(), typ: Type::Fighting});
            tmp_pokemons.insert(67, Pokemon {number: 67, name: "Machoke".into(), typ: Type::Fighting});
            tmp_pokemons.insert(68, Pokemon {number: 68, name: "Machamp".into(), typ: Type::Fighting});
            tmp_pokemons.insert(69, Pokemon {number: 69, name: "Bellsprout".into(), typ: Type::Grass});
            tmp_pokemons.insert(70, Pokemon {number: 70, name: "Weepinbell".into(), typ: Type::Grass});
            tmp_pokemons.insert(71, Pokemon {number: 71, name: "Victreebel".into(), typ: Type::Grass});
            tmp_pokemons.insert(72, Pokemon {number: 72, name: "Tentacool".into(), typ: Type::Water});
            tmp_pokemons.insert(73, Pokemon {number: 73, name: "Tentacruel".into(), typ: Type::Water});
            tmp_pokemons.insert(74, Pokemon {number: 74, name: "Geodude".into(), typ: Type::Rock});
            tmp_pokemons.insert(75, Pokemon {number: 75, name: "Graveler".into(), typ: Type::Rock});
            tmp_pokemons.insert(76, Pokemon {number: 76, name: "Golem".into(), typ: Type::Rock});
            tmp_pokemons.insert(77, Pokemon {number: 77, name: "Ponyta".into(), typ: Type::Fire});
            tmp_pokemons.insert(78, Pokemon {number: 78, name: "Rapidash".into(), typ: Type::Fire});
            tmp_pokemons.insert(79, Pokemon {number: 79, name: "Slowpoke".into(), typ: Type::Water});
            tmp_pokemons.insert(80, Pokemon {number: 80, name: "Slowbro".into(), typ: Type::Water});
            tmp_pokemons.insert(81, Pokemon {number: 81, name: "Magnemite".into(), typ: Type::Electric});
            tmp_pokemons.insert(82, Pokemon {number: 82, name: "Magneton".into(), typ: Type::Electric});
            tmp_pokemons.insert(83, Pokemon {number: 83, name: "Farfetchd".into(), typ: Type::Normal});
            tmp_pokemons.insert(84, Pokemon {number: 84, name: "Doduo".into(), typ: Type::Normal});
            tmp_pokemons.insert(85, Pokemon {number: 85, name: "Dodrio".into(), typ: Type::Normal});
            tmp_pokemons.insert(86, Pokemon {number: 86, name: "Seel".into(), typ: Type::Water});
            tmp_pokemons.insert(87, Pokemon {number: 87, name: "Dewgong".into(), typ: Type::Water});
            tmp_pokemons.insert(88, Pokemon {number: 88, name: "Grimer".into(), typ: Type::Poison});
            tmp_pokemons.insert(89, Pokemon {number: 89, name: "Muk".into(), typ: Type::Poison});
            tmp_pokemons.insert(90, Pokemon {number: 90, name: "Shellder".into(), typ: Type::Water});
            tmp_pokemons.insert(91, Pokemon {number: 91, name: "Cloyster".into(), typ: Type::Water});
            tmp_pokemons.insert(92, Pokemon {number: 92, name: "Gastly".into(), typ: Type::Ghost});
            tmp_pokemons.insert(93, Pokemon {number: 93, name: "Haunter".into(), typ: Type::Ghost});
            tmp_pokemons.insert(94, Pokemon {number: 94, name: "Gengar".into(), typ: Type::Ghost});
            tmp_pokemons.insert(95, Pokemon {number: 95, name: "Onix".into(), typ: Type::Rock});
            tmp_pokemons.insert(96, Pokemon {number: 96, name: "Drowzee".into(), typ: Type::Psychic});
            tmp_pokemons.insert(97, Pokemon {number: 97, name: "Hypno".into(), typ: Type::Psychic});
            tmp_pokemons.insert(98, Pokemon {number: 98, name: "Krabby".into(), typ: Type::Water});
            tmp_pokemons.insert(99, Pokemon {number: 99, name: "Kingler".into(), typ: Type::Water});
            tmp_pokemons.insert(100, Pokemon {number: 100, name: "Voltorb".into(), typ: Type::Electric});
            tmp_pokemons.insert(101, Pokemon {number: 101, name: "Electrode".into(), typ: Type::Electric});
            tmp_pokemons.insert(102, Pokemon {number: 102, name: "Exeggcute".into(), typ: Type::Grass});
            tmp_pokemons.insert(103, Pokemon {number: 103, name: "Exeggutor".into(), typ: Type::Grass});
            tmp_pokemons.insert(104, Pokemon {number: 104, name: "Cubone".into(), typ: Type::Ground});
            tmp_pokemons.insert(105, Pokemon {number: 105, name: "Marowak".into(), typ: Type::Ground});
            tmp_pokemons.insert(106, Pokemon {number: 106, name: "Hitmonlee".into(), typ: Type::Fighting});
            tmp_pokemons.insert(107, Pokemon {number: 107, name: "Hitmonchan".into(), typ: Type::Fighting});
            tmp_pokemons.insert(108, Pokemon {number: 108, name: "Lickitung".into(), typ: Type::Normal});
            tmp_pokemons.insert(109, Pokemon {number: 109, name: "Koffing".into(), typ: Type::Poison});
            tmp_pokemons.insert(110, Pokemon {number: 110, name: "Weezing".into(), typ: Type::Poison});
            tmp_pokemons.insert(111, Pokemon {number: 111, name: "Rhyhorn".into(), typ: Type::Ground});
            tmp_pokemons.insert(112, Pokemon {number: 112, name: "Rhydon".into(), typ: Type::Ground});
            tmp_pokemons.insert(113, Pokemon {number: 113, name: "Chansey".into(), typ: Type::Normal});
            tmp_pokemons.insert(114, Pokemon {number: 114, name: "Tangela".into(), typ: Type::Grass});
            tmp_pokemons.insert(115, Pokemon {number: 115, name: "Kangaskhan".into(), typ: Type::Normal});
            tmp_pokemons.insert(116, Pokemon {number: 116, name: "Horsea".into(), typ: Type::Water});
            tmp_pokemons.insert(117, Pokemon {number: 117, name: "Seadra".into(), typ: Type::Water});
            tmp_pokemons.insert(118, Pokemon {number: 118, name: "Goldeen".into(), typ: Type::Water});
            tmp_pokemons.insert(119, Pokemon {number: 119, name: "Seaking".into(), typ: Type::Water});
            tmp_pokemons.insert(120, Pokemon {number: 120, name: "Staryu".into(), typ: Type::Water});
            tmp_pokemons.insert(121, Pokemon {number: 121, name: "Starmie".into(), typ: Type::Water});
            tmp_pokemons.insert(122, Pokemon {number: 122, name: "Mr-mime".into(), typ: Type::Psychic});
            tmp_pokemons.insert(123, Pokemon {number: 123, name: "Scyther".into(), typ: Type::Bug});
            tmp_pokemons.insert(124, Pokemon {number: 124, name: "Jynx".into(), typ: Type::Ice});
            tmp_pokemons.insert(125, Pokemon {number: 125, name: "Electabuzz".into(), typ: Type::Electric});
            tmp_pokemons.insert(126, Pokemon {number: 126, name: "Magmar".into(), typ: Type::Fire});
            tmp_pokemons.insert(127, Pokemon {number: 127, name: "Pinsir".into(), typ: Type::Bug});
            tmp_pokemons.insert(128, Pokemon {number: 128, name: "Tauros".into(), typ: Type::Normal});
            tmp_pokemons.insert(129, Pokemon {number: 129, name: "Magikarp".into(), typ: Type::Water});
            tmp_pokemons.insert(130, Pokemon {number: 130, name: "Gyarados".into(), typ: Type::Water});
            tmp_pokemons.insert(131, Pokemon {number: 131, name: "Lapras".into(), typ: Type::Water});
            tmp_pokemons.insert(132, Pokemon {number: 132, name: "Ditto".into(), typ: Type::Normal});
            tmp_pokemons.insert(133, Pokemon {number: 133, name: "Eevee".into(), typ: Type::Normal});
            tmp_pokemons.insert(134, Pokemon {number: 134, name: "Vaporeon".into(), typ: Type::Water});
            tmp_pokemons.insert(135, Pokemon {number: 135, name: "Jolteon".into(), typ: Type::Electric});
            tmp_pokemons.insert(136, Pokemon {number: 136, name: "Flareon".into(), typ: Type::Fire});
            tmp_pokemons.insert(137, Pokemon {number: 137, name: "Porygon".into(), typ: Type::Normal});
            tmp_pokemons.insert(138, Pokemon {number: 138, name: "Omanyte".into(), typ: Type::Rock});
            tmp_pokemons.insert(139, Pokemon {number: 139, name: "Omastar".into(), typ: Type::Rock});
            tmp_pokemons.insert(140, Pokemon {number: 140, name: "Kabuto".into(), typ: Type::Rock});
            tmp_pokemons.insert(141, Pokemon {number: 141, name: "Kabutops".into(), typ: Type::Rock});
            tmp_pokemons.insert(142, Pokemon {number: 142, name: "Aerodactyl".into(), typ: Type::Rock});
            tmp_pokemons.insert(143, Pokemon {number: 143, name: "Snorlax".into(), typ: Type::Normal});
            tmp_pokemons.insert(144, Pokemon {number: 144, name: "Articuno".into(), typ: Type::Ice});
            tmp_pokemons.insert(145, Pokemon {number: 145, name: "Zapdos".into(), typ: Type::Electric});
            tmp_pokemons.insert(146, Pokemon {number: 146, name: "Moltres".into(), typ: Type::Fire});
            tmp_pokemons.insert(147, Pokemon {number: 147, name: "Dratini".into(), typ: Type::Dragon});
            tmp_pokemons.insert(148, Pokemon {number: 148, name: "Dragonair".into(), typ: Type::Dragon});
            tmp_pokemons.insert(149, Pokemon {number: 149, name: "Dragonite".into(), typ: Type::Dragon});
            tmp_pokemons.insert(150, Pokemon {number: 150, name: "Mewtwo".into(), typ: Type::Psychic});
            tmp_pokemons.insert(151, Pokemon {number: 151, name: "Mew".into(), typ: Type::Psychic});

            self.all_pokemons = tmp_pokemons;
        }

    }
}
