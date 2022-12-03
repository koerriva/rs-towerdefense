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
    pub now:Vec3,
    pub old:Vec3
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Bullet>()
        .register_type::<Lifetime>()
        .add_system(bullet_move)
        .add_system(bullet_destroy)
        .add_system(bullet_collision);
    }
}

fn bullet_move(
    mut commands:Commands,
    mut query:Query<(&mut Transform, &mut Bullet)>
){
    for (mut transform,mut bullet) in query.iter_mut(){
        let dv = (bullet.now - bullet.old) * 1.0;//friction
        bullet.old = bullet.now;
        bullet.now += dv;
        // bullet.now.y += -0.0002;//gravity

        transform.translation = bullet.now;
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
    mut bullets:Query<(Entity,&GlobalTransform),With<Bullet>>,
    mut targets:Query<(&GlobalTransform,&mut Health),With<Target>>
){
    for (bullet,bullet_transform) in bullets.iter_mut()  {
        for (target_transform,mut health) in targets.iter_mut() {
            if bullet_transform.translation().distance_squared(target_transform.translation()) < 0.1 {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                break;
            }
        }
    }
}