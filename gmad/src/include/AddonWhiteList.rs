use bootil::bstring::BString;
use bootil::string::test::wildcard;

pub mod addon {
    pub mod whitelist {
        pub static WILDCARD: &[&str] = &[
            "lua/*.lua",
            "scenes/*.vcd",
            "particles/*.pcf",
            "resource/fonts/*.ttf",
            "scripts/vehicles/*.txt",
            "resource/localization/*/*.properties",
            "maps/*.bsp",
            "maps/*.nav",
            "maps/*.ain",
            "maps/thumb/*.png",
            "sound/*.wav",
            "sound/*.mp3",
            "sound/*.ogg",
            "materials/*.vmt",
            "materials/*.vtf",
            "materials/*.png",
            "materials/*.jpg",
            "materials/*.jpeg",
            "models/*.mdl",
            "models/*.vtx",
            "models/*.phy",
            "models/*.ani",
            "models/*.vvd",
            "gamemodes/*/*.txt",
            "gamemodes/*/*.fgd",
            "gamemodes/*/logo.png",
            "gamemodes/*/icon24.png",
            "gamemodes/*/gamemode/*.lua",
            "gamemodes/*/entities/effects/*.lua",
            "gamemodes/*/entities/weapons/*.lua",
            "gamemodes/*/entities/entities/*.lua",
            "gamemodes/*/backgrounds/*.png",
            "gamemodes/*/backgrounds/*.jpg",
            "gamemodes/*/backgrounds/*.jpeg",
            "gamemodes/*/content/models/*.mdl",
            "gamemodes/*/content/models/*.vtx",
            "gamemodes/*/content/models/*.phy",
            "gamemodes/*/content/models/*.ani",
            "gamemodes/*/content/models/*.vvd",
            "gamemodes/*/content/materials/*.vmt",
            "gamemodes/*/content/materials/*.vtf",
            "gamemodes/*/content/materials/*.png",
            "gamemodes/*/content/materials/*.jpg",
            "gamemodes/*/content/materials/*.jpeg",
            "gamemodes/*/content/scenes/*.vcd",
            "gamemodes/*/content/particles/*.pcf",
            "gamemodes/*/content/resource/fonts/*.ttf",
            "gamemodes/*/content/scripts/vehicles/*.txt",
            "gamemodes/*/content/resource/localization/*/*.properties",
            "gamemodes/*/content/maps/*.bsp",
            "gamemodes/*/content/maps/*.nav",
            "gamemodes/*/content/maps/*.ain",
            "gamemodes/*/content/maps/thumb/*.png",
            "gamemodes/*/content/sound/*.wav",
            "gamemodes/*/content/sound/*.mp3",
            "gamemodes/*/content/sound/*.ogg",
        ];

        pub fn check(strname: &BString) -> bool {
            for wildcard in WILDCARD {
                if wildcard.is_empty() {
                    break;
                }

                if wildcard::wildcard(wildcard, strname) {
                    return true;
                }
            }

            false
        }
    }
}