use bevy::prelude::*;

#[derive(Resource,Clone)]
pub struct GameAssets{
    pub tower_base:Handle<Scene>,
    pub tower_base_mesh:Handle<Mesh>,
    pub tower_base_material:Handle<StandardMaterial>,
    pub tower_bottom:Handle<Scene>,
    pub weapon_cannon:Handle<Scene>,
    pub weapon_cannon_img:Handle<Image>,
    pub weapon_ballista:Handle<Scene>,
    pub weapon_ballista_img:Handle<Image>,
    pub weapon_blaster:Handle<Scene>,
    pub weapon_blaster_img:Handle<Image>,
    pub cannon_bullet:Handle<Scene>,
    pub ballista_bullet:Handle<Scene>,
    pub blaster_bullet:Handle<Scene>,
    pub cannon_fire_audio:Handle<AudioSource>,
    pub blaster_fire_audio:Handle<AudioSource>,
    pub ballista_fire_audio:Handle<AudioSource>,
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

    let cannon_fire_audio = asset.load("audio/Cannon.wav");
    let blaster_fire_audio = asset.load("audio/Blaster_short.wav");
    let ballista_fire_audio = asset.load("audio/Bow_Fire_Arrow.wav");
    
    let weapon_cannon = asset.load("models/weapon_cannon.glb#Scene0");
    let weapon_cannon_img = asset.load("images/weapon_cannon.png");

    let weapon_ballista = asset.load("models/weapon_ballista.glb#Scene0");
    let weapon_ballista_img = asset.load("images/weapon_ballista.png");

    let weapon_blaster = asset.load("models/weapon_blaster.glb#Scene0");
    let weapon_blaster_img = asset.load("images/weapon_blaster.png");


    let cannon_bullet = asset.load("models/cannon_bullet.glb#Scene0");
    let ballista_bullet = asset.load("models/ballista_bullet.glb#Scene0");
    let blaster_bullet = asset.load("models/blaster_bullet.glb#Scene0");

    let enemy_red = asset.load("models/enemy_ufoRed.glb#Scene0");
    let enemy_move_audio = asset.load("audio/spaceEngineLow_000.ogg");
    commands.insert_resource(GameAssets{
        tower_base,
        tower_base_mesh,tower_base_material,tower_bottom,
        weapon_cannon,weapon_cannon_img,
        weapon_ballista,weapon_ballista_img,
        weapon_blaster,weapon_blaster_img,
        cannon_bullet,ballista_bullet,blaster_bullet,
        cannon_fire_audio,blaster_fire_audio,ballista_fire_audio,
        enemy_red,enemy_move_audio
    });
}