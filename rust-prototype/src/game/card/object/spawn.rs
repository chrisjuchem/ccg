use crate::game::card::object::proto::CardProto;
use crate::game::card::object::zone::{InZone, Zone};
use crate::game::card::object::Card;
use crate::game::grab::{Draggable, Grabbable};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_mod_picking::prelude::{Bubble, Drop, ListenedEvent, OnPointer};
use bevy_text_mesh::prelude::*;

#[derive(Resource)]
pub struct CardAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    font: Handle<TextMeshFont>,
}

#[derive(SystemParam)]
pub struct CardSpawner<'w, 's> {
    materials: Res<'w, Assets<StandardMaterial>>,
    meshes: Res<'w, Assets<Mesh>>,
    card_assets: Res<'w, CardAssets>,
    commands: Commands<'w, 's>,
}

impl<'w, 's> CardSpawner<'w, 's> {
    pub fn spawn(&mut self, proto: CardProto, zone: Zone) -> Entity {
        let mut root = self.commands.spawn((
            InZone {
                zone,
                rel_order: proto.cost.mana.clone() as i16,
            },
            Card { proto },
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.5),
                ..default()
            },
            Draggable::new(),
        ));
        let root_id = root.id();

        root.with_children(|cmds| {
            cmds.spawn((
                PbrBundle {
                    mesh: self.card_assets.mesh.clone(),
                    material: self.card_assets.material.clone(),
                    ..default()
                },
                Grabbable::new(root_id),
            ));
            cmds.spawn(TextMeshBundle {
                text_mesh: TextMesh {
                    text: "AA BB CC DD EE FF GG HH II JJ KK LL MM NN OO PP QQ RR SS TT UU VV WW XX YY ZZ".into(),
                    style: TextMeshStyle {
                        font: self.card_assets.font.clone(),
                        font_size: SizeUnit::NonStandard(22.),
                        font_style: Default::default(),
                        color: Color::BLACK,
                        mesh_quality: Default::default()
                    },
                    alignment: TextMeshAlignment {
                        vertical: bevy_text_mesh::prelude::VerticalAlign::Top,
                        horizontal: bevy_text_mesh::prelude::HorizontalAlign::Center,
                    },
                    size: TextMeshSize{
                        width: SizeUnit::NonStandard(200.),
                        height: SizeUnit::NonStandard(100.),
                        depth: Some(SizeUnit::NonStandard(1.)),
                        wrapping: true,
                        overflow: true
                    }
                },

                // Card width = 3 (transform units) * 72 (textmesh scalar factor) = 216
                transform: Transform::from_xyz(-200. / 216. * 1.5, 0., 0.),
                ..default()
            });
        });

        root_id
    }
}

pub fn init_card_assets(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2 { x: 3., y: 4. })));
    let material = materials.add(StandardMaterial {
        perceptual_roughness: 0.7,
        ..Color::rgb(0.8, 0.7, 0.6).into()
    });
    let font = asset_server.load("fonts/FiraMono-Medium.ttf#mesh");

    commands.insert_resource(CardAssets {
        mesh,
        material,
        font,
    })
}
