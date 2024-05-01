use bevy::prelude::*;

use crate::gameplay::progression::per_run::RunProgression;
use crate::gameplay::world::table::*;

pub fn sit_at_table_from_progress(
    mut event_writer: EventWriter<SitAtTable>,
    run_progression: Res<RunProgression>,
) {
    let coordinates = run_progression.table_coordinates;
    event_writer.send(SitAtTable { coordinates: coordinates });
}