// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use anyhow::Result;

    use png2wasm4src::{build_sprite_modules_tree, Module};

    #[test]
    fn sprite_modules_tree() -> Result<()> {
        let module = build_sprite_modules_tree(Path::new("tests/sprites"))?;

        let expected = Module::new(
            "sprites",
            Vec::default(),
            vec![
                Module::new(
                    "characters",
                    vec![PathBuf::from("tests/sprites/characters/player.png")],
                    vec![
                        Module::new(
                            "npcs",
                            vec![
                                PathBuf::from("tests/sprites/characters/npcs/blacksmith.png"),
                                PathBuf::from("tests/sprites/characters/npcs/vendor.png"),
                            ],
                            Vec::default(),
                        ),
                        Module::new(
                            "bosses",
                            vec![
                                PathBuf::from("tests/sprites/characters/bosses/dragon.png"),
                                PathBuf::from("tests/sprites/characters/bosses/behemoth.png"),
                            ],
                            Vec::default(),
                        ),
                    ],
                ),
                Module::new(
                    "tiles",
                    vec![
                        PathBuf::from("tests/sprites/tiles/forest.png"),
                        PathBuf::from("tests/sprites/tiles/town.png"),
                        PathBuf::from("tests/sprites/tiles/desert.png"),
                    ],
                    Vec::default(),
                ),
            ],
        );

        assert_eq!(module, expected);

        Ok(())
    }

    #[test]
    fn sprite_modules_tree_to_string() -> Result<()> {
        let module = build_sprite_modules_tree(Path::new("tests/sprites"))?;
        let module = module.parse()?;
        let code = module.to_string();

        let expected = "pub mod sprites {
    pub mod characters {
        pub const PLAYER_WIDTH: u32 = 4;
        pub const PLAYER_HEIGHT: u32 = 4;
        pub const PLAYER_FLAGS: u32 = 1; // BLIT_2BPP
        pub const PLAYER: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

        pub mod bosses {
            pub const BEHEMOTH_WIDTH: u32 = 4;
            pub const BEHEMOTH_HEIGHT: u32 = 4;
            pub const BEHEMOTH_FLAGS: u32 = 1; // BLIT_2BPP
            pub const BEHEMOTH: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

            pub const DRAGON_WIDTH: u32 = 4;
            pub const DRAGON_HEIGHT: u32 = 4;
            pub const DRAGON_FLAGS: u32 = 1; // BLIT_2BPP
            pub const DRAGON: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

        }

        pub mod npcs {
            pub const BLACKSMITH_WIDTH: u32 = 4;
            pub const BLACKSMITH_HEIGHT: u32 = 4;
            pub const BLACKSMITH_FLAGS: u32 = 1; // BLIT_2BPP
            pub const BLACKSMITH: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

            pub const VENDOR_WIDTH: u32 = 4;
            pub const VENDOR_HEIGHT: u32 = 4;
            pub const VENDOR_FLAGS: u32 = 1; // BLIT_2BPP
            pub const VENDOR: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

        }

    }

    pub mod tiles {
        pub const DESERT_WIDTH: u32 = 4;
        pub const DESERT_HEIGHT: u32 = 4;
        pub const DESERT_FLAGS: u32 = 1; // BLIT_2BPP
        pub const DESERT: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

        pub const FOREST_WIDTH: u32 = 4;
        pub const FOREST_HEIGHT: u32 = 4;
        pub const FOREST_FLAGS: u32 = 1; // BLIT_2BPP
        pub const FOREST: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

        pub const TOWN_WIDTH: u32 = 4;
        pub const TOWN_HEIGHT: u32 = 4;
        pub const TOWN_FLAGS: u32 = 1; // BLIT_2BPP
        pub const TOWN: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

    }

}

";

        assert_eq!(code, expected);

        Ok(())
    }

    #[test]
    fn sprite_modules_list() -> Result<()> {
        let module = build_sprite_modules_tree(Path::new("tests/sprites"))?;
        let module = module.flatten();

        let expected = Module::new(
            "sprites",
            vec![
                PathBuf::from("tests/sprites/characters/player.png"),
                PathBuf::from("tests/sprites/characters/npcs/blacksmith.png"),
                PathBuf::from("tests/sprites/characters/npcs/vendor.png"),
                PathBuf::from("tests/sprites/characters/bosses/dragon.png"),
                PathBuf::from("tests/sprites/characters/bosses/behemoth.png"),
                PathBuf::from("tests/sprites/tiles/forest.png"),
                PathBuf::from("tests/sprites/tiles/town.png"),
                PathBuf::from("tests/sprites/tiles/desert.png"),
            ],
            Vec::default(),
        );

        assert_eq!(module, expected);

        Ok(())
    }

    #[test]
    fn sprite_modules_list_to_string() -> Result<()> {
        let module = build_sprite_modules_tree(Path::new("tests/sprites"))?;
        let module = module.flatten();
        let module = module.parse()?;
        let code = module.to_string();

        let expected = "pub mod sprites {
    pub const BEHEMOTH_WIDTH: u32 = 4;
    pub const BEHEMOTH_HEIGHT: u32 = 4;
    pub const BEHEMOTH_FLAGS: u32 = 1; // BLIT_2BPP
    pub const BEHEMOTH: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

    pub const BLACKSMITH_WIDTH: u32 = 4;
    pub const BLACKSMITH_HEIGHT: u32 = 4;
    pub const BLACKSMITH_FLAGS: u32 = 1; // BLIT_2BPP
    pub const BLACKSMITH: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

    pub const DESERT_WIDTH: u32 = 4;
    pub const DESERT_HEIGHT: u32 = 4;
    pub const DESERT_FLAGS: u32 = 1; // BLIT_2BPP
    pub const DESERT: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

    pub const DRAGON_WIDTH: u32 = 4;
    pub const DRAGON_HEIGHT: u32 = 4;
    pub const DRAGON_FLAGS: u32 = 1; // BLIT_2BPP
    pub const DRAGON: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

    pub const FOREST_WIDTH: u32 = 4;
    pub const FOREST_HEIGHT: u32 = 4;
    pub const FOREST_FLAGS: u32 = 1; // BLIT_2BPP
    pub const FOREST: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

    pub const PLAYER_WIDTH: u32 = 4;
    pub const PLAYER_HEIGHT: u32 = 4;
    pub const PLAYER_FLAGS: u32 = 1; // BLIT_2BPP
    pub const PLAYER: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

    pub const TOWN_WIDTH: u32 = 4;
    pub const TOWN_HEIGHT: u32 = 4;
    pub const TOWN_FLAGS: u32 = 1; // BLIT_2BPP
    pub const TOWN: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

    pub const VENDOR_WIDTH: u32 = 4;
    pub const VENDOR_HEIGHT: u32 = 4;
    pub const VENDOR_FLAGS: u32 = 1; // BLIT_2BPP
    pub const VENDOR: [u8; 4] = [0x5a, 0x5a, 0xf0, 0xf0];

}

";

        assert_eq!(code, expected);

        Ok(())
    }
}
