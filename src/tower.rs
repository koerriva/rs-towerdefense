use std::f32::consts::PI;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::audio::*;
use bevy::transform;
use bevy::utils::FloatOrd;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::Selection;
use leafwing_input_manager::orientation::Orientation;

use crate::assets::*;
use crate::target::*;
use crate::bullet::*;

#[derive(Component,Reflect,Default)]
#[reflect(Component)]
pub struct Tower{
    pub shooting_timer:Timer
}

#[derive(Inspectable,Component,Clone,Copy,Debug)]
pub enum TowerType{
    Cannon,Ballista,Blaster
} 

impl TowerType {
    pub fn get_tower(&self,assets:&GameAssets) -> (Handle<Scene>,Tower) {
        match self {
            //加农炮
            TowerType::Cannon => (
                assets.weapon_cannon.clone(),
                Tower{ shooting_timer: Timer::from_seconds(2.0, TimerMode::Repeating) }
            ),
            //投射机
            TowerType::Ballista => (
                assets.weapon_ballista.clone(),
                Tower{ shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating) }
            ),
            //能量武器
            TowerType::Blaster => (
                assets.weapon_blaster.clone(),
                Tower{ shooting_timer: Timer::from_seconds(0.25, TimerMode::Repeating) }
            )
        }
    }

    pub fn get_bullet(&self,offset:Vec3,bullet_dir:Vec3,time:&Time,assets:&GameAssets) -> (Handle<Scene>,Bullet) {
        match self {
            TowerType::Cannon => (
                assets.cannon_bullet.clone(),
                Bullet{ 
                    new:offset+bullet_dir*time.delta_seconds()*12.5,old:offset,
                    gravity_scalar:0.098, friction_scalar:0.998
                }
            ),
            TowerType::Ballista => (
                assets.ballista_bullet.clone(),
                Bullet{ 
                    new:offset+bullet_dir*time.delta_seconds()*7.5,old:offset,
                    gravity_scalar:0.0098, friction_scalar:1.002
                }
            ),
            TowerType::Blaster => (
                assets.blaster_bullet.clone(),
                Bullet{ 
                    new:offset+bullet_dir*time.delta_seconds()*20.,old:offset,
                    gravity_scalar:0., friction_scalar:1.0
                }
            ),
        }
    }

    pub fn get_sfx(&self,assets:&GameAssets) -> Handle<AudioSource> {
        match self {
            TowerType::Blaster => assets.blaster_fire_audio.clone(),
            TowerType::Ballista => assets.ballista_fire_audio.clone(),
            TowerType::Cannon => assets.cannon_fire_audio.clone(),
        }
    }
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Tower>()
        .add_system(tower_shooting);
    }
}

fn tower_shooting(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>,
    mut towers:Query<(Entity,&mut Tower,&TowerType,&GlobalTransform,&Transform)>,
    targets:Query<&GlobalTransform,With<Target>>,
    time:Res<Time>,
    assets:Res<GameAssets>,
    audio:Res<Audio>
){
    if targets.iter().count() < 1 {
        return;
    }
    for (e,mut tower,tower_type,global_tansform,local_transform) in towers.iter_mut() {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            //bullet
            let spawn_offset = Vec3::new(0.,0.25,0.);
            let bullet_spawn_position = global_tansform.translation() + spawn_offset;
            let mut bullet_dir = Vec3::ZERO;

            if let Some(closest_target) = targets.iter().min_by_key(|target_transform|{
                FloatOrd(target_transform.translation().distance(bullet_spawn_position))
            }) {
                //dir
                bullet_dir = local_transform.rotation * (closest_target.translation() - bullet_spawn_position).normalize();
            }

            let (bullet_model,bullet) = tower_type.get_bullet(spawn_offset,bullet_dir,&time,&assets);

            commands.entity(e).with_children(|cb|{
                cb.spawn(SceneBundle{
                    scene:bullet_model.clone(),
                    transform:Transform { 
                        translation: spawn_offset, 
                        scale: Vec3::new(0.05,0.05,0.05),
                        rotation: Quat::IDENTITY
                    },
                    ..default()
                })
                .insert(Lifetime{timer:Timer::from_seconds(5., TimerMode::Once)})
                .insert(bullet)
                .insert(Name::new("Bullet"));
            });
            
            // audio.play(assets.cannon_fire_audio.clone());
            audio.play_with_settings(tower_type.get_sfx(&assets), PlaybackSettings { repeat: false, volume: 0.4, speed: 1.0 });
        }
    }
}