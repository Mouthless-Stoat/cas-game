use bevy::prelude::*;

macro_rules! texture_enum {
    ($($n:ident),*) => {
        #[derive(Clone, Copy)]
        pub enum Texture {
            $($n),*
        }

        impl From<Texture> for usize {
            fn from(val: Texture) -> Self {
                match val {
                    $(Texture::$n => ${index()}),*
                }
            }
        }
    };
}

texture_enum! {
    Player, Blank, Dwarf,
    Snake, Zombie, Goblin,
    Ground, Brick, Soil, Grass, Flower, Grass2, FlowerPatch
}

#[derive(Resource)]
pub struct Atlast {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

impl Atlast {
    #[must_use]
    pub fn new(texture: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Atlast { texture, layout }
    }

    #[must_use]
    pub fn as_sprite_data(&self, texture: Texture) -> (Handle<Image>, TextureAtlas) {
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.layout.clone(),
                index: texture.into(),
            },
        )
    }
}

#[derive(Bundle)]
pub struct AtlastSpriteBundle {
    atlast: AtlastSprite,
    sprite: Sprite,
}

impl AtlastSpriteBundle {
    #[must_use]
    pub fn new(texture: Texture) -> Self {
        AtlastSpriteBundle {
            atlast: AtlastSprite(texture),
            sprite: default(),
        }
    }
}

#[derive(Component)]
pub struct AtlastSprite(pub Texture);

pub fn atlast_to_sprite(atlast: Res<Atlast>, mut query: Query<(&mut Sprite, &AtlastSprite)>) {
    for (mut s, AtlastSprite(t)) in &mut query {
        if let Some(a) = &mut s.texture_atlas {
            a.index = (*t).into();
        } else {
            let (i, t) = atlast.as_sprite_data(*t);
            s.image = i;
            s.texture_atlas = Some(t);
        }
    }
}
