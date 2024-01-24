use std::{fs::File, path::PathBuf, time::Duration};

use bevy::prelude::*;
use serde::Serialize;

use super::{artefact_paths::ArtefactPaths, FirstUpdate, TestQuitEvent};

#[derive(Debug, Default, Resource)]
struct FrameCollector(Vec<Duration>);

const OUTLIERS: usize = 10;

impl FrameCollector {
    fn write_to(&self, path: PathBuf) {
        let mut sum = Duration::default();
        let mut best = vec![];
        let mut worst = vec![];

        // For some reason, first one has a delta time of 0.0
        for frame in self.0.iter().skip(1) {
            sum += *frame;

            if best.len() < OUTLIERS {
                best.push(*frame);
                worst.push(*frame);
                continue;
            }

            let best_of_worst = worst.iter().min().unwrap().to_owned();
            let worst_of_best = best.iter().max().unwrap().to_owned();

            if *frame > best_of_worst {
                worst.retain(|f| f != &best_of_worst);
                worst.push(*frame);
            } else if *frame < worst_of_best {
                best.retain(|f| f != &worst_of_best);
                best.push(*frame);
            }
        }

        best.sort();
        worst.sort();

        let metrics = FrameMetrics {
            frames: to_millivec(self.0.clone()),
            average: 1000.0 * sum.as_secs_f32() / self.0.len() as f32,
            best: to_millivec(best),
            worst: to_millivec(worst),
        };

        let mut file = File::create(path).unwrap();

        serde_json::to_writer_pretty(&mut file, &metrics).unwrap();
    }
}

fn to_millivec(durations: Vec<Duration>) -> Vec<f32> {
    durations
        .into_iter()
        .map(|f| 1000.0 * f.as_secs_f32())
        .collect()
}

#[derive(Debug, Default, Serialize)]
struct FrameMetrics {
    average: f32,
    best: Vec<f32>,
    worst: Vec<f32>,
    frames: Vec<f32>,
}

#[derive(Debug)]
pub struct FrameMetricPlugin;

impl Plugin for FrameMetricPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, record_frame_metrics)
            .add_systems(
                Update,
                write_frame_metrics.run_if(on_event::<TestQuitEvent>()),
            )
            .init_resource::<FrameCollector>();
    }
}

fn record_frame_metrics(
    mut frame_metrics: ResMut<FrameCollector>,
    time: Res<Time<Real>>,
    first_update: Option<Res<FirstUpdate>>,
) {
    if first_update.is_some() {
        frame_metrics.0.push(time.delta());
    }
}

fn write_frame_metrics(
    frame_metrics: Res<FrameCollector>,
    artefact_paths: Res<ArtefactPaths>,
    mut test_quit_events: EventReader<TestQuitEvent>,
) {
    if test_quit_events.read().next().is_some() {
        frame_metrics.write_to(artefact_paths.frame_metrics());
    }
}
