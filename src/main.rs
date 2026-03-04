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
        .init_resource::<BGMState>()
        .init_resource::<AudioLoadState>()
        //.init_resource::<SoundsFolder>()
        .init_resource::<SoundLoader>()
        .add_systems(Startup, setup_title)
        .add_systems(Update, (
            audio_button,
            load_state,
            ))
        .run();
}

#[component]
fn App() -> impl IntoView {
    let start = std::sync::Once::new();
    view!{
        <button
            on:click = move|_| {
                start.call_once(|| setup() )
            }
        >
        "START"
        </button>
        <canvas
        id="bevy_audio"
        style="width:auto;height:100vh;"
        />
    }
}

fn main() {
    mount_to_body(App);
}

#[derive(Resource, Default)]
struct AudioLoadState {
    sound_loading: bool,
    ui_updated: bool,
}

//#[derive(Resource, Default)]
//struct SoundsFolder (Handle<bevy::asset::LoadedFolder>);

#[derive(Resource, Default)]
struct SoundLoader {
    bgm: Handle<AudioSource>,
    start: Handle<AudioSource>,
    open_cell: Handle<AudioSource>,
}

#[derive(Resource, Default)]
struct BGMState {
    entity: Option<Entity>,
}

#[derive(Component)]
struct LoadStateText;

#[derive(Component)]
enum ButtonType {
    Bgm,
    OpenCell,
    Start,
}

use crate::ButtonType::{Bgm, OpenCell, Start};
fn setup_title( mut commands: Commands, asset_server: Res<AssetServer>, mut sound_loader: ResMut<SoundLoader> ) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Percent(25.),
            left: Val::Percent(50.),
            width: Val::Px(48.),
            height: Val::Px(48.),
            ..default()
        },
        BackgroundColor (Color::srgb(0.,0.,0.)),
        Text::new("Sound Loading"),
        LoadStateText,
    ));

    sound_loader.bgm = asset_server.load("sounds/bgm.ogg");
    sound_loader.start = asset_server.load("sounds/start.mp3");
    sound_loader.open_cell = asset_server.load("sounds/open_cell.wav");

    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Percent(50.),
            left: Val::Percent(36.),
            width: Val::Px(32.),
            height: Val::Px(32.),
            ..default()
        },
        BackgroundColor (Color::srgb(0.,0.,1.)),
        Button,
        Bgm,
        Text::new("ogg")
        ));

    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Percent(50.),
            left: Val::Percent(50.),
            width: Val::Px(32.),
            height: Val::Px(32.),
            ..default()
        },
        BackgroundColor (Color::srgb(1.,0.,1.)),
        Button,
        OpenCell,
        Text::new("wav")
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Percent(50.),
            left: Val::Percent(64.),
            width: Val::Px(32.),
            height: Val::Px(32.),
            ..default()
        },
        BackgroundColor (Color::srgb(0.,1.,1.)),
        Button,
        Start,
        Text::new("mp3")
    ));
}

use bevy::ecs::query::With;

fn audio_button(
    mut commands: Commands,
    //mut sounds_folder: ResMut<SoundsFolder>,
    sound_loader: ResMut<SoundLoader>,
    audio_load_state: ResMut<AudioLoadState>,
    mut bgmstate: ResMut<BGMState>,
    mut sounds_query: Query<(&Interaction, &ButtonType, &mut BackgroundColor), (With<Button>, Changed<Interaction>)>,
) {
    for (ints, button, mut bgcolor) in &mut sounds_query {
        match *ints {
            // SoundLoader
            Interaction::Pressed => {
                match button {

                    Bgm => {
                        //if !asset_server.is_loaded_with_dependencies(sounds_folder.0.id()) {
                        //    continue
                        //}
                        if audio_load_state.ui_updated == false {
                            continue
                        }
                        match bgmstate.entity {
                            Some(entity) => {
                                commands.entity(entity).despawn();
                                bgmstate.entity = None;
                            }
                            None => {
                                let entity = commands.spawn((AudioPlayer::new(sound_loader.bgm.clone()), PlaybackSettings::LOOP)).id();
                                bgmstate.entity = Some(entity);
                            }
                        }
                    }

                    Start => {
                        //if !asset_server.is_loaded_with_dependencies(sounds_folder.0.id()) {
                        //    continue
                        //}
                        if audio_load_state.ui_updated == false {
                            continue
                        }
                        commands.spawn((AudioPlayer::new(sound_loader.start.clone()), PlaybackSettings::DESPAWN));
                    }

                    OpenCell => {
                        //if !asset_server.is_loaded_with_dependencies(sounds_folder.0.id()) {
                        //    continue
                        //}
                        if audio_load_state.ui_updated == false {
                            continue
                        }
                        commands.spawn((AudioPlayer::new(sound_loader.open_cell.clone()), PlaybackSettings::DESPAWN));
                    }
                }
            }

            Interaction::Hovered => {
                match button {
                    Bgm => {
                        *bgcolor = BackgroundColor(Color::srgb(1., 0., 0.));
                    }
                    Start => {
                        *bgcolor = BackgroundColor(Color::srgb(0., 1., 1.));
                    }
                    OpenCell => {
                        *bgcolor = BackgroundColor(Color::srgb(0., 1., 0.));
                    }
                }
            }

            Interaction::None => {
                match button {
                    Bgm => {
                        *bgcolor = BackgroundColor(Color::srgb(0., 0., 1.));
                    }
                    Start => {
                        *bgcolor = BackgroundColor(Color::srgb(1., 1., 0.));
                    }
                    OpenCell => {
                        *bgcolor = BackgroundColor(Color::srgb(1., 0., 1.));
                    }
                }
            }
        }
    }
}

fn load_state(
    asset_server: Res<AssetServer>,
    //sounds_folder: Res<SoundsFolder>,
    sound_loader: Res<SoundLoader>,
    mut audio_load_state: ResMut<AudioLoadState>,
    mut load_state_query: Query<(&mut BackgroundColor, &mut Text), With<LoadStateText>>,
) {
    if !audio_load_state.ui_updated {
        audio_load_state.sound_loading = true;
        if asset_server.is_loaded(&sound_loader.bgm)
            && asset_server.is_loaded(&sound_loader.start)
            && asset_server.is_loaded(&sound_loader.open_cell) {

            //if asset_server.is_loaded_with_dependencies(sounds_folder.0.id()) && audio_load_state.sound_loaded == true {
            for (mut bgcolor, mut text) in &mut load_state_query {
                *bgcolor = BackgroundColor(Color::srgb(0.,1.,0.));
                **text = "Loaded!".to_string();
                audio_load_state.ui_updated = true;
            }
            //}
        }

    }
}