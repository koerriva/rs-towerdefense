use std::f32::consts::PI;

use bevy::{prelude::*, ecs::query::QuerySingleError};
use bevy_inspector_egui::*;
use bevy_mod_picking::*;

use crate::{GameAssets, TowerType};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_startup_system(create_ui)
        .add_system(tower_button_clicked)
        .add_system(create_ui_on_selected);
    }
}

#[derive(Component)]
pub struct TowerRootUI;

fn create_ui(
    mut commands:Commands,
    assets:Res<GameAssets>
){
    let types = [
        (assets.weapon_ballista_img.clone(),TowerType::Ballista),
        (assets.weapon_blaster_img.clone(),TowerType::Blaster),
        (assets.weapon_cannon_img.clone(),TowerType::Cannon)
    ];

    commands.spawn(NodeBundle{
        style:Style { 
            size:Size::new(Val::Percent(100.),Val::Percent(100.)),
            justify_content:JustifyContent::Center,
            ..default()
        },
        background_color:Color::NONE.into(),
        ..default()
    })
    .insert(TowerRootUI)
    .with_children(|cb|{
        for i in 0..3 {
            cb.spawn(ButtonBundle{
                style:Style{
                    size:Size::new(Val::Percent(10.*9.0/16.0), Val::Percent(10.*9.0/16.0)),
                    align_self:AlignSelf::FlexStart,
                    margin:UiRect::all(Val::Percent(2.)),
                    ..default()
                },
                background_color:Color::WHITE.into(),
                image:types[i].0.clone().into(),
                ..default()
            })
            .insert(types[i].1);
        }
    });
}

fn tower_button_clicked(
    mut commands:Commands,
    selection:Query<(Entity,&Selection,&Transform)>,
    assets:Res<GameAssets>,
    query:Query<(&Interaction,&TowerType),Changed<Interaction>>
){
    for (interaction,tower_type) in query.iter() {
        if matches!(interaction,Interaction::Clicked) {
            info!("spawn tower!");
            for (e,selection,transform) in selection.iter() {
                if selection.selected() {
                    commands.entity(e).despawn_recursive();
                    spawn_tower(&mut commands, &assets, tower_type.clone(),transform.translation,transform.rotation);
                }
            }
        }
    }
}

fn spawn_tower(
    commands:&mut Commands,
    assets:&GameAssets,
    tower_type:TowerType,
    position:Vec3,
    rotation:Quat,
) -> Entity {

    let (tower_model,tower) = tower_type.get_tower(assets);

    commands.spawn(SceneBundle{
        scene: assets.tower_base.clone(),
        transform: Transform{
            translation: position, 
            rotation: Quat::IDENTITY, 
            scale: Vec3::ONE 
        },
        ..default()
    })
    .insert(Name::new("TowerBase"))
    .with_children(|cb|{
        cb.spawn(SceneBundle{
            scene:tower_model.clone(),
            transform:Transform { 
                translation: Vec3::new(0.0,0.15,0.), 
                rotation: rotation, 
                scale: Vec3::ONE  
            },
            ..default()
        })
        .insert(tower)
        .insert(tower_type)
        .insert(Name::new("Tower"));
    })
    .id()
}

fn create_ui_on_selected(
    mut commands:Commands,
    assets:Res<GameAssets>,
    selection:Query<&Selection>,
    root:Query<Entity,With<TowerRootUI>>
){
    let any_selection = selection.iter().any(|s|{s.selected()});
    match root.get_single() {
        Ok(e) => {
            if !any_selection {
                info!("hide ui");
                commands.entity(e).despawn_recursive();
            }
        },
        Err(QuerySingleError::NoEntities(..)) => {
            if any_selection {
                create_ui(commands, assets);
            }
        },
        Err(..) => {
            error!("too many ui roots!")
        }
    }
}