use std::f32::consts::PI;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike,PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Move,MoveForward,MoveBackward,MoveLeft,MoveRight
}

#[derive(Component)]
pub struct PlayerInput;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_controller);
    }
}

fn camera_controller(
    keyboard:Res<Input<KeyCode>>,
    mut cameras:Query<&mut Transform,With<Camera3d>>,
    actions:Query<&ActionState<Action>,With<PlayerInput>>,
    time:Res<Time>
){
    let mut camera = cameras.get_single_mut().expect("can't find a camera!");

    let mut forward = camera.forward();
    forward.y = 0.;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.;
    left = left.normalize();

    for action in actions.iter() {
        let dir = action.clamped_axis_pair(Action::Move).unwrap().xy();
        camera.translation += forward * time.delta_seconds() * dir.y;
        camera.translation -= left * time.delta_seconds() * dir.x;
    }


    // if keyboard.pressed(KeyCode::W) {
    //     camera.translation += forward * time.delta_seconds()  * 1.5;
    // }

    // if keyboard.pressed(KeyCode::S) {
    //     camera.translation -= forward * time.delta_seconds()  * 1.5;
    // }

    // if keyboard.pressed(KeyCode::A) {
    //     camera.translation += left * time.delta_seconds() * 1.5;
    // }

    // if keyboard.pressed(KeyCode::D) {
    //     camera.translation -= left * time.delta_seconds() * 1.5;
    // }

    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_y(time.delta_seconds() * PI * 0.1);
    }

    if keyboard.pressed(KeyCode::E) {
        camera.rotate_y(time.delta_seconds() * PI * -0.1);
    }
}