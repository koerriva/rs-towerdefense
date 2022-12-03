use bevy::prelude::*;
use bevy::audio::*;

#[derive(Resource,Clone)]
pub struct GameAssets{
    pub tower_base:Handle<Scene>,
    pub tower_base_mesh:Handle<Mesh>,
    pub tower_base_material:Handle<StandardMaterial>,
    pub tower_bottom:Handle<Scene>,
    pub weapon_cannon:Handle<Scene>,
    pub cannon_fire_audio:Handle<AudioSource>,
    pub enemy_red:Handle<Scene>,
    pub enemy_move_audio:Handle<AudioSource>,
}

pub struct GameAssetsPlugin;
impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
    }
}

fn load_assets(
    mut commands:Commands,
    asset:Res<AssetServer>
){
    let tower_base = asset.load("models/towerRound_base.glb#Scene0");
    let tower_base_mesh = asset.load("models/towerRound_base_mesh.glb#Mesh0/Primitive0");
    let tower_base_material = asset.load("models/towerRound_base_mesh.glb#Material0");
    let tower_bottom = asset.load("models/towerRound_bottomA.glb#Scene0");
    let cannon_fire_audio = asset.load("audio/cannon_02.ogg");
    let weapon_cannon = asset.load("models/weapon_cannon.glb#Scene0");

    let enemy_red = asset.load("models/enemy_ufoRed.glb#Scene0");
    let enemy_move_audio = asset.load("audio/spaceEngineLow_000.ogg");
    commands.insert_resource(GameAssets{
        tower_base,
        tower_base_mesh,tower_base_material,tower_bottom,
        weapon_cannon,cannon_fire_audio,enemy_red,enemy_move_audio
    });
}