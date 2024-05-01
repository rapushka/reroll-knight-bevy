use bevy::prelude::EventWriter;

use crate::gameplay::progression::per_run::StartRun;

pub fn write_start_run_event(
    mut event_writer: EventWriter<StartRun>
) {
    event_writer.send(StartRun {});
}