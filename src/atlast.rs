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
    Ground1,
    Ground2,
    Ground3,
    PlayerL,
    PlayerR,
    PlayerU,
    PlayerD,
    Blank
}

#[derive(Resource)]
pub struct Atlast {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

impl Atlast {
    pub fn new(texture: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Atlast { texture, layout }
    }

    pub fn get_sprite(&self, index: Texture) -> Sprite {
        Sprite::from_atlas_image(
            self.texture.clone(),
            TextureAtlas {
                layout: self.layout.clone(),
                index: index.into(),
            },
        )
    }
}

#[derive(Component)]
pub struct AtlastSprite(pub Texture);

pub fn atlast_to_sprite(
    mut commands: Commands,
    atlast: Res<Atlast>,
    query: Query<(Entity, &AtlastSprite)>,
) {
    for (e, a) in &query {
        let mut e = commands.entity(e);
        e.insert(atlast.get_sprite(a.0));
    }
}
