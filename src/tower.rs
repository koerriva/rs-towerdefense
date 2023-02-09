use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::FloatOrd;
use bevy_inspector_egui::Inspectable;

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

    pub fn get_bullet(&self,position:Vec3,bullet_dir:Vec3,time:&Time,assets:&GameAssets) -> (Handle<Scene>,Bullet) {
        match self {
            TowerType::Cannon => (
                assets.cannon_bullet.clone(),
                Bullet{ 
                    new:position+bullet_dir*time.delta_seconds()*12.5,old:position,
                    gravity_scalar:0.098, friction_scalar:0.9998,
                    is_missile:false,
                    damage:10
                }
            ),
            TowerType::Ballista => (
                assets.ballista_bullet.clone(),
                Bullet{ 
                    new:position+bullet_dir*time.delta_seconds()*8.5,old:position,
                    gravity_scalar:0.0098, friction_scalar:1.0002,
                    is_missile:false,
                    damage:1
                }
            ),
            TowerType::Blaster => (
                assets.blaster_bullet.clone(),
                Bullet{ 
                    new:position+bullet_dir*time.delta_seconds()*20.,old:position,
                    gravity_scalar:0., friction_scalar:1.0,
                    is_missile:false,
                    damage:1
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
    // mut meshes:ResMut<Assets<Mesh>>,
    // mut materials:ResMut<Assets<StandardMaterial>>,
    mut towers:Query<(Entity,&mut Tower,&TowerType,&GlobalTransform,&mut Transform)>,
    mut bullet_collectors:Query<Entity,With<BulletCollector>>,
    targets:Query<(&GlobalTransform,&Target),Without<TargetDeath>>,
    time:Res<Time>,
    assets:Res<GameAssets>,
    audio:Res<Audio>
){
    if targets.iter().count() < 1 {
        return;
    }
    for (e,mut tower,tower_type,global_tansform,mut local_transform) in towers.iter_mut() {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            //bullet
            let spawn_offset = Vec3::new(0.,0.25,0.);
            let bullet_spawn_position = global_tansform.translation() + spawn_offset;
            let forward = -Vec3::Z;

            if let Some((closest_target,_)) = targets.iter().min_by_key(|(target_transform,_)|{
                FloatOrd(target_transform.translation().distance(bullet_spawn_position))
            }) {
                let pos = closest_target.translation() - bullet_spawn_position;
                let target_pos = pos+Vec3::Y*0.1+Vec3::X*0.1;
                local_transform.look_at(target_pos, Vec3::Y);
            }

            let (bullet_model,bullet) = tower_type.get_bullet(bullet_spawn_position,local_transform.rotation * forward,&time,&assets);

            // commands.entity(e).with_children(|cb|{
                
            // });

            match bullet_collectors.get_single_mut() {
                Ok(collector) => {
                    commands.entity(collector).with_children(|cb|{
                        cb.spawn(SceneBundle{
                            scene:bullet_model.clone(),
                            transform:Transform { 
                                translation: bullet_spawn_position, 
                                scale: Vec3::new(0.05,0.05,0.05),
                                rotation: local_transform.rotation
                            },
                            ..default()
                        })
                        .insert(Lifetime{timer:Timer::from_seconds(5., TimerMode::Once)})
                        .insert(bullet)
                        .insert(Name::new("Bullet"));
                    });
                },
                Err(_) => todo!(),
            }
            
            // audio.play(assets.cannon_fire_audio.clone());
            audio.play_with_settings(tower_type.get_sfx(&assets), PlaybackSettings { repeat: false, volume: 0.4, speed: 1.0 });
        }
    }
}