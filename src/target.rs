use std::f32::consts::PI;

use bevy::prelude::*;
use rand::random;
use crate::assets::*;

#[derive(Component,Reflect,Default)]
#[reflect(Component)]
pub struct TargetFactory{
    pub spawn_timer:Timer,
}

#[derive(Component,Reflect,Default)]
#[reflect(Component)]
pub struct Target{
    pub speed:f32,
    pub sfx:Handle<AudioSource>
}

#[derive(Component,Reflect,Default)]
#[reflect(Component)]
pub struct Health{
    pub value:i32
}

#[derive(Component)]
pub struct TargetDeath;

pub struct TargetPlugin;

impl Plugin for TargetPlugin  {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Target>()
        .register_type::<Health>()
        .register_type::<TargetFactory>()
        .add_system(target_spawn)
        .add_system(target_move)
        .add_system(target_death);
    }
}

fn target_spawn(
    mut commands:Commands,
    mut factories:Query<&mut TargetFactory>,
    assets:Res<GameAssets>,
    time:Res<Time>
){
    let speed:f32 = random();
    for mut factory in factories.iter_mut() {
        factory.spawn_timer.tick(time.delta());

        if factory.spawn_timer.finished() {
            commands.spawn(SceneBundle{
                scene:assets.enemy_red.clone(),
                transform:Transform{
                    translation:Vec3::new(-4.,speed+0.5,0.),
                    scale:Vec3::new(0.5,0.5,0.5),
                    ..default()
                },
                ..default()
            })
            .insert(Target{speed:speed+1.0,sfx:assets.enemy_move_audio.clone()})
            .insert(Health{value:10})
            .insert(Name::new("Target"));
        }
    }
}

fn target_move(
    mut commands:Commands,
    mut alive_queue:Query<(Entity,&Target,&mut Transform),Without<TargetDeath>>,
    mut death_queue:Query<(Entity,&Target,&mut Transform),With<TargetDeath>>,
    // assets:Res<GameAssets>,
    // audio_skin:Res<Assets<AudioSink>>,
    // audio:Res<Audio>,
    time:Res<Time>
){
    for (e,_,mut transform) in death_queue.iter_mut() {
        transform.translation.y -= 0.98 * time.delta_seconds();
        transform.rotate_x(time.delta_seconds()*PI);
        transform.rotate_y(time.delta_seconds()*PI);

        if transform.translation.y < 0. {
            commands.entity(e).despawn_recursive();
            info!("target fallout!")
        }
    }

    for (e,target,mut transform) in alive_queue.iter_mut() {
        transform.translation.x += target.speed * time.delta_seconds();

        if transform.translation.x >= 4. {
            commands.entity(e).despawn_recursive();
            info!("target destroy!")
        }
    }
}

fn target_death(
    mut commands:Commands,
    query:Query<(Entity,&Health,&Target),Without<TargetDeath>>
){
    for (e,health,_) in query.iter() {
        if health.value <=0 {
            commands.entity(e).insert(TargetDeath);
            info!("target death!")
        }
    }
}