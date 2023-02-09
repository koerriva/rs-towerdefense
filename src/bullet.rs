use std::f32::consts::PI;

use bevy::prelude::*;
use crate::target::*;

#[derive(Component,Reflect,Default)]
#[reflect(Component)]
pub struct Lifetime{
    pub timer:Timer
}

#[derive(Component,Reflect,Default)]
#[reflect(Component)]
pub struct Bullet{
    pub old:Vec3,
    pub new:Vec3,
    pub gravity_scalar:f32,//重力系数
    pub friction_scalar:f32,//摩擦力系数
    pub is_missile:bool,
    pub damage:i32,
}

#[derive(Component,Reflect,Default)]
#[reflect(Component)]
pub struct BulletCollector;

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Bullet>()
        .register_type::<Lifetime>()
        .add_startup_system(bullet_collect)
        .add_system(bullet_move)
        .add_system(bullet_destroy)
        .add_system(bullet_collision);
    }
}

fn bullet_collect(
    mut commands:Commands
){
    commands
    .spawn(SpatialBundle::default())
    .insert(BulletCollector)
    .insert(Name::new("BulletCollector"));
}

fn bullet_move(
    // mut commands:Commands,
    mut bullets:Query<(&mut Transform, &mut Bullet)>,
    time:Res<Time>
){
    for (mut transform,mut bullet) in bullets.iter_mut() {
        let dv = (bullet.new - bullet.old) * bullet.friction_scalar;

        bullet.old = bullet.new;
        bullet.new += dv;
        bullet.new.y -= time.delta_seconds() * bullet.gravity_scalar;


        let up = Vec3::Y;
        transform.look_at(bullet.new, up);
        transform.translation = bullet.new;
    }
}

fn bullet_destroy(
    mut commands:Commands,
    mut query:Query<(Entity,&mut Lifetime),With<Bullet>>,
    time:Res<Time>
){
    for (e,mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn bullet_collision(
    mut commands:Commands,
    mut bullets:Query<(Entity,&GlobalTransform,&Bullet),With<Bullet>>,
    mut targets:Query<(&GlobalTransform,&mut Health),With<Target>>
){
    for (bullet_e,bullet_transform,bullet) in bullets.iter_mut()  {
        for (target_transform,mut health) in targets.iter_mut() {
            if bullet_transform.translation().distance(target_transform.translation()) < 0.2 {
                commands.entity(bullet_e).despawn_recursive();
                health.value -= bullet.damage;
                break;
            }
        }
    }
}