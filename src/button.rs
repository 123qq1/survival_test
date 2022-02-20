use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Default)]
pub struct MapCompile;

#[derive(Component,Default)]
struct MapButton;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin{
    fn build(&self, app: &mut App){
        compile_button::<MapButton,MapCompile>(app);
    }
}

fn compile_button<C: Component + Default, E: 'static + Send + Sync + Default>(
    app :&mut App,
){
    app.add_event::<E>();
    app.add_system(button_system::<C,E>.system());
    app.add_startup_system(button_setup::<C>.system());
}

fn button_setup<C: Component + Default>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect {
                    left: Val::Px(8.0),
                    bottom: Val::Px(8.0),
                    right: Val::Auto,
                    top: Val::Auto,
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            //color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(C::default())
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

fn button_system<T: Component, E: 'static + Send + Sync + Default>(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>, With<T>),
    >,
    mut text_query: Query<&mut Text>,
    mut but_ev: EventWriter<E>,
    //
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();

                but_ev.send(E::default());
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}