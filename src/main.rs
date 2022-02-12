use bevy::prelude::*;

const CAM_SCALE : f32 = 0.5;
const GRID_HEIGHT_PADDING: f32 = 8.0;
const GRID_WIDTH_PADDING: f32 = 16.0;
const GRID_WIDTH_OFFSET: f32 = 16.0;
const GRID_HEIGHT_OFFSET: f32 = 8.0;
const GRID_WIDTH_SIZE: i32 = 10;
const GRID_HEIGHT_SIZE: i32 = 10;


fn main() {
    App::build()
        //Window
        .insert_resource(WindowDescriptor{
            title: "Survival Test".to_string(),
            width: 800.0,
            height: 600.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.75,0.95,1.0)))

        //Prerequisites
        .add_plugins(DefaultPlugins)

        //Startup
        .add_startup_system(setup.system())

        .run();
}

struct MainCamera;
struct BlockId {
    block_id: i32,
    index_id: i32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let texture_handle = asset_server.load("atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0,32.0),1,1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut cam_bundle = OrthographicCameraBundle::new_2d();

    cam_bundle.orthographic_projection.scale = CAM_SCALE;
    cam_bundle.transform.translation.y = -40.0;
    commands.spawn_bundle(cam_bundle).insert(MainCamera);

    let mut i = 0;

    for y in 0..GRID_WIDTH_SIZE {
        for x in 0..GRID_HEIGHT_SIZE {
            let x_pos = -x as f32 * GRID_WIDTH_PADDING  + GRID_WIDTH_OFFSET * y as f32;
            let y_pos = -x as f32 * GRID_HEIGHT_PADDING - GRID_HEIGHT_OFFSET * y as f32;
            let z_order = 500.0 - y_pos;
            commands.spawn()
                .insert_bundle(SpriteSheetBundle{
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(x_pos,y_pos , z_order)),
                    sprite:TextureAtlasSprite::new(0),
                    ..Default::default()
                })
                .insert(BlockId{
                    block_id: 0,
                    index_id: i,
                });
            i += 1;
        }
    }
}

