use crate::{game_state::*, utils::despawn_entities};
use bevy::prelude::*;

use super::GameTimer;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(spawn_ui))
            .add_system_set(
                SystemSet::on_exit(GameState::Play).with_system(despawn_entities::<Hud>),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Play).with_system(change_game_timer_hud),
            );
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct XpBar;

#[derive(Component)]
struct GameTimerHud;

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Hud)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(3.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                            ..Default::default()
                        },
                        color: Color::BEIGE.into(),
                        ..Default::default()
                    });
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(30.0), Val::Percent(40.0)), //XP bar at 40% currently
                                ..Default::default()
                            },
                            color: Color::BLUE.into(),
                            ..Default::default()
                        })
                        .insert(XpBar);
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                            ..Default::default()
                        },
                        color: Color::BEIGE.into(),
                        ..Default::default()
                    });
                });
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(5.0),
                            right: Val::Px(15.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    color: Color::BEIGE,
                                },
                            },
                            TextSection {
                                value: "seconds".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    color: Color::BEIGE,
                                },
                            },
                        ],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(GameTimerHud);
        });
}

fn change_game_timer_hud(time: Res<GameTimer>, mut query: Query<&mut Text, With<GameTimerHud>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:.1} ", time.0);
    }
}
