use bevy::prelude::*;
use crate::block::BlockId;

#[derive(Component)]
pub struct MainCamera;

pub struct MouseAsset{
    pos: Vec3,
}

#[derive(Component)]
pub struct MainSelect;

pub struct InputPlugin;

impl Plugin for InputPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(cam_control.system());
        app.add_startup_system(input_setup.system());

        app.add_system(main_select_closest.system());
        app.add_system(mouse_screen_world_pos.system());
    }
}

const SCROLL_SPEED: f32 = 8.0;
const ZOOM_SPEED: f32 = 0.02;

fn input_setup(
    mut commands: Commands,
){
    commands.insert_resource(MouseAsset{pos:Vec3::new(0.0,0.0,0.0)});
}

fn cam_control(
    input : Res<Input<KeyCode>>,
    mut cam_query: Query<(&mut Transform,&mut OrthographicProjection), With<MainCamera>>
){
    let (mut cam_t,mut cam_p) = cam_query.single_mut();

    let scale = cam_p.scale;

    if input.pressed(KeyCode::A) {
        cam_t.translation.x += -SCROLL_SPEED * scale;
    }
    if input.pressed(KeyCode::D) {
        cam_t.translation.x += SCROLL_SPEED * scale;
    }
    if input.pressed(KeyCode::W) {
        cam_t.translation.y += SCROLL_SPEED * scale;
    }

    if input.pressed(KeyCode::S) {
        cam_t.translation.y -= SCROLL_SPEED * scale;
    }

    if input.pressed(KeyCode::Q) {
        if scale > 0.1 {
            cam_p.scale -= ZOOM_SPEED;
        }
    }

    if input.pressed(KeyCode::E) {
        cam_p.scale += ZOOM_SPEED;
    }
}

fn mouse_screen_world_pos(
    window: Res<Windows>,
    cam_qery: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_pos: ResMut<MouseAsset>,
){
    let (cam, cam_trans) = cam_qery.single();

    let win = window.get(cam.window).unwrap();

    if let Some(screen_pos) = win.cursor_position(){
        let window_size = Vec2::new(win.width() as f32, win.height() as f32);

        let ndc = (screen_pos/window_size)*2.0 - Vec2::ONE;

        let ndc_to_world = cam_trans.compute_matrix() * cam.projection_matrix.inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        let world_pos: Vec2 = world_pos.truncate();

        mouse_pos.pos = Vec3::from((world_pos,0.0));
    }
}

fn main_select_closest(
    mut query: QuerySet<(
        QueryState<(&mut GlobalTransform, &BlockId)>,
        QueryState<(&mut Transform, &mut Visibility), With<MainSelect>>
    )>,
    mouse_asset: Res<MouseAsset>,
){
    let mut d = 20.0;
    let mut v = mouse_asset.pos;

    for (b_trans,b_id) in query.q0().iter() {

        let new_d = mouse_asset.pos.distance(b_trans.translation);

        if new_d < d {
            d = new_d;
            v = b_trans.translation;
            v.y += b_id.height + 8.0;
        }
    }


    for (mut select_trans, mut vis) in query.q1().iter_mut() {

        if d == 20.0 {
            vis.is_visible = false;
        }
        else{
            vis.is_visible = true;
        }

        select_trans.translation = v;
    }


}


