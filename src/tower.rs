use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::audio::*;
use bevy::utils::FloatOrd;
use bevy_mod_picking::Selection;

use crate::assets::*;
use crate::target::*;
use crate::bullet::*;

#[derive(Component,Reflect,Default)]
#[reflect(Component)]
pub struct Tower{
    pub shooting_timer:Timer
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Tower>()
        .add_system(tower_shooting)
        .add_system(build_tower);
    }
}

fn tower_shooting(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>,
    mut towers:Query<(Entity,&mut Tower,&GlobalTransform)>,
    targets:Query<&GlobalTransform,With<Target>>,
    time:Res<Time>,
    assets:Res<GameAssets>,
    audio:Res<Audio>
){
    if targets.iter().count() < 1 {
        return;
    }
    for (e,mut tower,global_tansform) in towers.iter_mut() {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            //bullet
            let spawn_offset = Vec3::new(0.,0.25,0.);
            let bullet_spawn_position = global_tansform.translation() + spawn_offset;
            let mut bullet_dir = Vec3::Z;

            if let Some(closest_target) = targets.iter().min_by_key(|target_transform|{
                FloatOrd(target_transform.translation().distance(bullet_spawn_position))
            }) {
                //rotate
                // local_transform.look_at(closest_target.translation(), Vec3::Y);
                //dir
                bullet_dir = (closest_target.translation() - bullet_spawn_position).normalize_or_zero() * time.delta_seconds()*5.;
            }

            //fix FORWAR
            bullet_dir *= Vec3::new(-1., 1., -1.);

            commands.entity(e).with_children(|cb|{
                cb.spawn(PbrBundle{
                    transform:Transform { translation: spawn_offset, scale: Vec3::new(0.05,0.05,0.05),..default() },
                    mesh:meshes.add(Mesh::from(shape::Icosphere::default())),
                    material:materials.add(StandardMaterial { 
                        base_color:Color::BLACK,
                        emissive:Color::ORANGE_RED,
                        ..default()
                    }),
                    ..default()
                })
                .insert(Lifetime{timer:Timer::from_seconds(10., TimerMode::Once)})
                .insert(Bullet{now:spawn_offset+bullet_dir,old:spawn_offset})
                .insert(Name::new("Bullet"));
            });
            
            // audio.play(assets.cannon_fire_audio.clone());
            audio.play_with_settings(assets.cannon_fire_audio.clone(), PlaybackSettings { repeat: false, volume: 0.4, speed: 1.0 });
        }
    }
}

fn build_tower(
    mut commands:Commands,
    selection:Query<(Entity,&Selection,&Transform)>,
    keyboard:Res<Input<KeyCode>>,
    assets:Res<GameAssets>
){
    if keyboard.just_pressed(KeyCode::Space) {
        for (e,selection,transform) in selection.iter() {
            if selection.selected() {
                commands.entity(e).despawn_recursive();
                spawn_cannon_tower(&mut commands, &assets, transform.translation);
            }
        }
    }
}

fn spawn_cannon_tower(
    commands:&mut Commands,
    assets:&GameAssets,
    position:Vec3,
) -> Entity {
    commands.spawn(SceneBundle{
        scene: assets.tower_base.clone(),
        transform: Transform{
            translation: position, 
            rotation: Quat::from_rotation_y(PI), 
            scale: Vec3::ONE 
        },
        ..default()
    })
    .insert(Name::new("TowerBase"))
    .with_children(|cb|{
        cb.spawn(SceneBundle{
            scene:assets.weapon_cannon.clone(),
            transform:Transform::from_xyz(0., 0.15, 0.),
            ..default()
        })
        .insert(Tower{shooting_timer:Timer::from_seconds(1.5, TimerMode::Repeating)})
        .insert(Name::new("Tower"));
    })
    .id()
}