use std::{
    fs::{create_dir_all, read_to_string, remove_dir_all, File},
    path::{Path, PathBuf},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use bevy::{
    app::AppExit,
    ecs::system::SystemId,
    prelude::*,
    render::view::screenshot::ScreenshotManager,
    window::{exit_on_all_closed, exit_on_primary_closed, PrimaryWindow},
};

use crate::AssertSystem;

#[derive(Debug, Resource)]
struct ScriptPath(PathBuf);

#[derive(Debug, Resource)]
struct ArtefactPath(PathBuf);

#[derive(Debug)]
pub struct PlaybackTestGear {
    path: String,
}

impl PlaybackTestGear {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Plugin for PlaybackTestGear {
    fn build(&self, app: &mut App) {
        let (maybe_script, script_path, artefact_path) = load_script(self.path.clone());

        if let Some(script) = maybe_script {
            app.insert_resource(script)
                .add_systems(First, script_player);

            let id = app.world.register_system(record_artefacts);
            app.insert_resource(QuitCallback(id));
            app
        } else {
            app.insert_resource(TestScript::default())
                .add_systems(First, script_recorder);

            let id = app.world.register_system(save_script);
            app.insert_resource(QuitCallback(id));
            app
        }
        .insert_resource(ScriptPath(script_path))
        .insert_resource(ArtefactPath(artefact_path))
        .add_event::<CustomQuitEvent>()
        .add_systems(
            PostUpdate,
            run_asserts
                .run_if(on_event::<AppExit>())
                .after(exit_on_primary_closed)
                .after(exit_on_all_closed),
        )
        .add_systems(Update, delayed_exit);
    }
}

#[derive(Debug, Resource)]
struct QuitCallback(SystemId);

#[derive(Debug, Clone, Serialize, Deserialize)]
enum UserInput {
    KeyPress(KeyCode),
    KeyRelese(KeyCode),
    Quit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Resource)]
struct TestScript(Vec<(Duration, UserInput)>);

fn load_script(case_name: String) -> (Option<TestScript>, PathBuf, PathBuf) {
    let base_path = Path::new("bitt");

    let script_path = base_path
        .join("test_scripts")
        .join(format!("{}.bitt_script", case_name));

    let artefact_path = base_path.join("artefacts").join(case_name);

    (
        if script_path.exists() {
            let file = read_to_string(&script_path).unwrap();
            let script: TestScript = serde_json::from_str(&file).unwrap();
            Some(script)
        } else {
            None
        },
        script_path,
        artefact_path,
    )
}

fn script_recorder(
    mut script: ResMut<TestScript>,
    time: Res<Time<Real>>,
    input: Res<Input<KeyCode>>,
) {
    for key in input.get_just_pressed() {
        script.0.push((time.elapsed(), UserInput::KeyPress(*key)));
    }

    for key in input.get_just_released() {
        script.0.push((time.elapsed(), UserInput::KeyRelese(*key)));
    }
}

fn save_script(script: Res<TestScript>, path: Res<ScriptPath>, time: Res<Time<Real>>) {
    let mut script = script.clone();
    script.0.push((time.elapsed(), UserInput::Quit));

    let prefix = path.0.parent().unwrap();
    create_dir_all(prefix).unwrap();
    let file = File::create(&path.0).unwrap();
    serde_json::to_writer(file, &script).unwrap();
}

#[derive(Debug, Clone, Copy, Event)]
struct CustomQuitEvent;

fn script_player(
    mut last_run: Local<Duration>,
    time: Res<Time<Real>>,
    script: Res<TestScript>,
    mut quit_events: EventWriter<CustomQuitEvent>,
    mut kb_input: ResMut<Input<KeyCode>>,
) {
    for ev in script
        .0
        .iter()
        .skip_while(|(event_time, _)| *event_time < *last_run)
        .take_while(|(event_time, _)| *event_time < time.elapsed())
        .map(|(_, input)| input)
    {
        match ev {
            UserInput::KeyPress(key) => kb_input.press(*key),
            UserInput::KeyRelese(key) => kb_input.release(*key),
            UserInput::Quit => quit_events.send(CustomQuitEvent),
        }
    }

    *last_run = time.elapsed();
}

#[derive(Debug, Clone, Resource)]
struct EndScreenshot(PathBuf);

fn record_artefacts(
    mut commands: Commands,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    path: Res<ArtefactPath>,
) {
    remove_dir_all(path.0.clone()).unwrap();
    create_dir_all(path.0.clone()).unwrap();
    let img_path = path.0.clone().join("screenshot.png");
    screenshot_manager
        .save_screenshot_to_disk(main_window.single(), img_path.clone())
        .unwrap();

    commands.insert_resource(EndScreenshot(img_path));
}

fn run_asserts(mut commands: Commands, assert_sys: Res<AssertSystem>) {
    commands.run_system(assert_sys.0);
}

fn delayed_exit(
    mut quit_events: ResMut<Events<AppExit>>,
    mut commands: Commands,
    callback: Res<QuitCallback>,
    custom_quit_events: EventReader<CustomQuitEvent>,
    mut started: Local<bool>,
    screenshot: Option<Res<EndScreenshot>>,
) {
    if *started {
        let mut done = false;

        if let Some(shot) = screenshot {
            if shot.0.exists()
                && File::open(shot.0.clone())
                    .unwrap()
                    .metadata()
                    .unwrap()
                    .len()
                    > 0
            {
                done = true;
            }
        }

        if done {
            quit_events.send(AppExit);
        }
    } else if !custom_quit_events.is_empty() {
        commands.run_system(callback.0);
        *started = true;
    }
}
