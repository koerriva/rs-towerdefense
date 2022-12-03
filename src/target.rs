use bevy::prelude::*;
use bevy::audio::*;
use crate::assets::*;

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

pub struct TargetPlugin;

impl Plugin for TargetPlugin  {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Target>()
        .register_type::<Health>()
        .add_system(target_move)
        .add_system(target_death);
    }
}

fn target_move(
    mut commands:Commands,
    mut query:Query<(Entity,&Target,&mut Transform)>,
    assets:Res<GameAssets>,
    audio_skin:Res<Assets<AudioSink>>,
    audio:Res<Audio>,
    time:Res<Time>
){
    for (e,target,mut transform) in query.iter_mut() {
        transform.translation.x += target.speed * time.delta_seconds();

        if transform.translation.x >= 4. {
            commands.entity(e).despawn_recursive();
            println!("target destroy!")
        }
    }
}

fn target_death(
    mut commands:Commands,
    query:Query<(Entity,&Health),With<Target>>
){
    for (e,health) in query.iter() {
        if health.value <=0 {
            commands.entity(e).despawn_recursive();
            println!("target death!")
        }
    }
}