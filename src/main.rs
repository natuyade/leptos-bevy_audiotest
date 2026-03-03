use bevy::prelude::*;
use leptos::prelude::*;

fn setup() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            canvas: Some("#bevy_audio".to_string()),
                            fit_canvas_to_parent: true,
                            ..default()
                        }
                    ),
                    ..default()
                }
            )
            .set(
                AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                }
            )
        )
        .add_systems(Startup, setup_title)
        .add_systems(Update, audio_button)
        .run();
}

#[component]
fn App() -> impl IntoView {
    view!{
        <canvas id="bevy_audio" style="width:auto;height:100vh;"/>
    }
}

fn main() {
    mount_to_body(App);
    setup();
}

#[derive(Resource)]
struct GameAudio {
    bgm: Handle<AudioSource>,
    cell_click: Handle<AudioSource>,
    start: Handle<AudioSource>,
    failed: Handle<AudioSource>,
    setting: Handle<AudioSource>
}

#[derive(Component)]
struct PlayingBGM (bool);

#[derive(Component)]
struct BGM;

fn setup_title( mut commands: Commands, asset_server: Res<AssetServer> ) {
    commands.spawn(Camera2d);

    commands.insert_resource(GameAudio{
        bgm: asset_server.load("sounds/bgm.wav"),
        cell_click: asset_server.load("sounds/open_cell.wav"),
        start: asset_server.load("sounds/start.wav"),
        failed: asset_server.load("sounds/failed.wav"),
        setting: asset_server.load("sounds/setting_button.wav"),
    });


    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Percent(50.),
            left: Val::Percent(50.),
            width: Val::Px(32.),
            height: Val::Px(32.),
            ..default()
        },
        BackgroundColor (Color::srgb(1.,1.,1.)),
        Button,
        ));

    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Percent(50.),
            left: Val::Percent(32.),
            width: Val::Px(32.),
            height: Val::Px(32.),
            ..default()
        },
        PlayingBGM { 0: false },
        Button,
        BackgroundColor (Color::srgb(0.,0.,1.)),
        ));
}

use bevy::ecs::query::With;

fn audio_button(
    audio: Res<GameAudio>,
    audio_assets: Res<Assets<AudioSource>>,
    mut commands: Commands,
    mut query: Query<(&Interaction, &mut BackgroundColor),(Without<PlayingBGM>, Changed<Interaction>)>,
    mut query2: Query<(&Interaction, &mut BackgroundColor, &mut PlayingBGM), (With<Button>, Changed<Interaction>)>,
    bgmentity: Query<Entity, With<BGM>>
) {
    for (ints, mut bgcolor, mut playing) in &mut query2 {

            match *ints {
                Interaction::Pressed => {
                    if audio_assets.get(&audio.bgm).is_none() {
                        *playing = PlayingBGM { 0: false };
                        return;
                    }
                    match playing.0 {
                        true => {
                            for entity in &bgmentity {
                                commands.entity(entity).despawn();
                            }
                            *playing = PlayingBGM { 0: false };
                        }
                        false => {
                            commands.spawn((AudioPlayer::new(audio.bgm.clone()), PlaybackSettings::LOOP, BGM));
                            *playing = PlayingBGM { 0: true };
                        }
                    }
                }
                Interaction::Hovered => {
                    *bgcolor = BackgroundColor(Color::srgb(1., 0., 0.));
                }
                Interaction::None => {
                    *bgcolor = BackgroundColor(Color::srgb(0., 0., 1.));
                }
            }

    }

    for (ints, mut bgcolor) in &mut query {
        match *ints {
            Interaction::Pressed => {
                if audio_assets.get(&audio.cell_click).is_none() {
                    return;
                }
                commands.spawn((AudioPlayer::new(audio.cell_click.clone()), PlaybackSettings::DESPAWN));
            }
            Interaction::Hovered => {
                *bgcolor = BackgroundColor(Color::srgb(0., 1., 0.));
            }
            Interaction::None => {
                *bgcolor = BackgroundColor(Color::srgb(1., 1., 1.));
            }
        }
    }
}
