use bevy::{prelude::*, audio::AudioSink, utils::{FloatOrd}, pbr::NotShadowCaster};
use bevy_inspector_egui::WorldInspectorPlugin;
use std::f32::consts::*;
use rand::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_mod_picking::*;
use bevy_hikari::prelude::*;
use bevy_atmosphere::prelude::*;

mod bullet;
mod target;
mod tower;
mod assets;
mod input;
mod ui;

pub use bullet::*;
pub use target::*;
pub use tower::*;
pub use assets::*;
pub use input::*;
pub use ui::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(
        WindowPlugin {
            window:WindowDescriptor{
                title:"Tower Defense".into(),
                width:1280.0,
                height:720.0,
                resizable:false,
                ..default()
            },
            ..default()
        }
    ))
    // .add_plugin(HikariPlugin)
    .add_plugin(AtmospherePlugin)
    .add_startup_system(setup)
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(GameAssetsPlugin)
    .add_plugin(TargetPlugin)
    .add_plugin(TowerPlugin)
    .add_plugin(BulletPlugin)
    .add_plugin(PlayerInputPlugin)
    .add_plugin(GameUIPlugin)
    .add_plugin(InputManagerPlugin::<Action>::default())
    .add_plugins(DefaultPickingPlugins)
    .run();
}

fn setup(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>,
    assets:Res<GameAssets>,
    audio:Res<Audio>,
    audio_skin:Res<Assets<AudioSink>>
){
    //camera
    commands.spawn(Camera3dBundle{
        transform:Transform::from_xyz(2.0,2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(PickingCameraBundle::default())
    .insert(HikariSettings{
        indirect_bounces:2,
        ..default()
    })
    .insert(AtmosphereCamera::default())
    .insert(Name::new("MainCamera"));

    //ground
    commands.spawn(PbrBundle{
        transform:Transform::from_xyz(0.0,0.0,0.0),
        mesh:meshes.add(Mesh::from(shape::Plane{size:50.0})),
        material:materials.add(StandardMaterial { 
            base_color:Color::DARK_GREEN,
            perceptual_roughness:0.998,
            ..default()
        }),
        ..default()
    })
    .insert(Name::new("Ground"));

    //tower
    for i in 0..3 {
        let x:f32 = -1.5+i as f32 * 1.5;
        let z:f32 = -1.;
        let speed:f32 = random();

        let default_color = materials.add(Color::rgba(0.3,0.5, 0.3, 0.3).into());
        let selected_color = materials.add(Color::rgba(0.3,0.9, 0.3, 0.9).into());

        commands
        .spawn(SpatialBundle::from_transform(Transform { 
            translation: Vec3::new(x,0.0,z), 
            rotation: Quat::from_rotation_y(PI), 
            scale: Vec3::ONE 
        }))
        // .insert(meshes.add(Mesh::from(shape::Capsule::default())))
        // .insert(meshes.add(Mesh::from(shape::Box::new(1.,1.,1.))))
        .insert(assets.tower_base_mesh.clone())
        // .insert(assets.tower_base_1.clone())
        .insert(default_color.clone())
        .insert(Highlighting{
            initial: default_color.clone(),
            hovered: Some(selected_color.clone()),
            pressed: Some(selected_color.clone()),
            selected: Some(selected_color.clone()),
        })
        .insert(NotShadowCaster)
        .insert(PickableBundle::default())
        .insert(Name::new("TowerBase"))
        .with_children(|cb|{
            cb.spawn(SceneBundle{
                scene:assets.tower_base.clone(),
                transform:Transform { translation: Vec3::ZERO, rotation: Quat::IDENTITY, scale: Vec3::new(0.99,0.99,0.99) },
                ..default()
            });
        });
    }
    

    //target factory
    commands.spawn(SpatialBundle::default())
    .insert(TargetFactory{ spawn_timer: Timer::from_seconds(3., TimerMode::Repeating) })
    .insert(Name::new("TowerFactory"));

    //light
    commands.spawn(DirectionalLightBundle{
        directional_light:DirectionalLight { 
            shadows_enabled:true,
            illuminance:5000.,
            ..default()
        },
        transform:Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    })
    .insert(Name::new("Sun"));

    //fogs
    //input
    commands.spawn(InputManagerBundle{
        action_state:ActionState::default(),
        input_map:InputMap::default()
        .set_gamepad(Gamepad{id:0})
        .insert(DualAxis::left_stick(), Action::Move)
        .insert(VirtualDPad::wasd(), Action::Move)
        .build()
    }).insert(PlayerInput);
}