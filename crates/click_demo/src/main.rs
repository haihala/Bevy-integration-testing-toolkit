use bevy::prelude::*;
use bitt::{Asserter, HeadlessDefaultPlugins, PlaybackTestGear, PlaybackTestingOptions};
use clap::{Parser, ValueEnum};
use click_demo::{ClickDemoPlugin, Points};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum IntegrationTestScript {
    ThreeClicks,
}
impl std::fmt::Display for IntegrationTestScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegrationTestScript::ThreeClicks => write!(f, "three-clicks"),
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the test script
    script: Option<IntegrationTestScript>,

    /// Run in headless or not
    #[arg(long)]
    headless: bool,

    /// Fails if a script is not recorded
    #[arg(long)]
    ci: bool,
}

fn main() {
    let args = Args::parse();
    let mut app = App::new();

    if args.headless {
        app.add_plugins(HeadlessDefaultPlugins);
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app.add_plugins(ClickDemoPlugin);

    match args.script {
        Some(script) => {
            app.add_plugins(PlaybackTestGear::new(
                script.to_string(),
                PlaybackTestingOptions {
                    read_only: args.ci,
                    ..default()
                },
            ));

            match script {
                IntegrationTestScript::ThreeClicks => {
                    app.add_systems(Update, assert_score_of_three);
                }
            }
        }
        _ => {
            assert!(!args.headless, "A script must be provided in headless mode");
            assert!(!args.ci, "A script must be provided in CI mode");
        }
    }

    app.run();
}

fn assert_score_of_three(score: Res<Points>, mut asserter: ResMut<Asserter>) {
    if score.0 == 3 {
        asserter.pass();
    }
}

// Add one test in headless mode that uses a faster time scale
