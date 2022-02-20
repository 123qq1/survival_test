mod world_gen;
mod button;
mod input;
mod block;

use bevy::prelude::*;
use bevy::ecs::component::Component;
use crate::KeyCode::D;

const CAM_SCALE : f32 = 1.2;
const GRID_HEIGHT_PADDING: f32 = 8.0;
const GRID_WIDTH_PADDING: f32 = 16.0;
const GRID_WIDTH_OFFSET: f32 = 16.0;
const GRID_HEIGHT_OFFSET: f32 = 8.0;
const GRID_WIDTH_SIZE: i32 = 10;
const GRID_HEIGHT_SIZE: i32 = 10;

fn main() {
    App::new()
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

        //Events
        .add_event::<GenerateMapEvent>()

        //Plugins
        .add_plugin(button::ButtonPlugin)
        .add_plugin(input::InputPlugin)

        //Startup
        .add_startup_system(setup.system())

        //Systems
        .add_system(generate_world.system())
        .add_system(compile_map_for_gen.system())

        .run();
}

#[derive(Component)]
struct Map;

struct WorldAssets{
    map: Handle<Image>,
    palette: Handle<Image>,
}

struct AtlasAsset{
    handle: Handle<TextureAtlas>,
}

struct GenerateMapEvent{
    map: Vec<usize>,
    height: u32,
    width: u32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let texture_handle = asset_server.load("atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0,32.0),3,2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut cam_bundle = OrthographicCameraBundle::new_2d();

    cam_bundle.orthographic_projection.scale = CAM_SCALE;
    cam_bundle.transform.translation.y = -150.0;
    commands.spawn_bundle(cam_bundle).insert(input::MainCamera);

    let palette: Handle<Image> = asset_server.load("palette.png");
    let world: Handle<Image> = asset_server.load("world.png");

    commands.insert_resource(WorldAssets{map: world,palette});
    commands.insert_resource(AtlasAsset{handle: texture_atlas_handle});

    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(SpriteBundle{
        transform: Transform::from_translation(Vec3::new(0.0,0.0,0.0)),
        texture: asset_server.load("main_select.png"),
        ..Default::default()

    })
        .insert(input::MainSelect);

}

fn generate_world(
    mut commands: Commands,
    texture_atlas: Res<AtlasAsset>,
    mut map_gen_ev: EventReader<GenerateMapEvent>,
    map_query: Query<Entity,With<Map>>
){
    for ev in map_gen_ev.iter() {
        for e in map_query.iter() {
            commands.entity(e).despawn_recursive();
        }

        let atlas_handle = texture_atlas.handle.clone();

        let map = &ev.map;
        let width = ev.width;
        let height = ev.height;

        let mut i = 0;

        let parent = commands.spawn()
            .insert(Map)
            //.insert(Transform::from_translation(Vec3::new(0.0,0.0,0.0)))
            .id();

        for y in 0..height {
            for x in 0..width {
                let x_pos = -(x as f32) * GRID_WIDTH_PADDING  + GRID_WIDTH_OFFSET * y as f32;
                let y_pos = -(x as f32) * GRID_HEIGHT_PADDING - GRID_HEIGHT_OFFSET * y as f32;
                let z_order = -y_pos/20.0;
                let map_id = map[i];

                let child = commands.spawn()
                    .insert_bundle(SpriteSheetBundle{
                        texture_atlas: atlas_handle.clone(),
                        global_transform: GlobalTransform::from_translation(Vec3::new(x_pos,y_pos , z_order)),
                        sprite:TextureAtlasSprite::new(map_id),
                        ..Default::default()
                    })
                    .insert(block::BlockId::from_values(map_id,i as i32,0.0)
                    ).id();

                commands.entity(parent).push_children(&[child]);
                i += 1;
            }
        }
    }
}

fn compile_map_for_gen(
    mut but_ev: EventReader<button::MapCompile>,
    world_assetes: Res<WorldAssets>,
    images: Res<Assets<Image>>,
    mut map_gen_ev: EventWriter<GenerateMapEvent>,
){
    for ev in but_ev.iter() {
        /*
        let map = images.get(world_assetes.map.clone()).unwrap();
        let palette = images.get(world_assetes.palette.clone()).unwrap();
        */
        let p_ls = world_gen::PassLayer::from_arr(&[(2,1,12,1),(0,2,6,2),(3,0,4,1),(5,3,1,1)]);

        let (map, width, height) = world_gen::world_from_seed(1, 40, 40, p_ls);

        map_gen_ev.send(GenerateMapEvent { map, height, width });
    }
}

