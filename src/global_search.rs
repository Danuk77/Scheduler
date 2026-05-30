use core::fmt;

use log::{error, info};
use rayon::prelude::*;

use crate::{
    constraints::constraint_store::ConstraintStore, hill_climber::run_hill_climber,
    schedule::Schedule, stats::OptimisationStats,
};

#[derive(Debug)]
pub enum GlobalSearchError {
    NoResults,
}

impl fmt::Display for GlobalSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GlobalSearchError::NoResults => {
                write!(f, "Global search failed to run all parallel searches")
            }
        }
    }
}

type HillClimbResult = (Schedule, u32, OptimisationStats);

pub fn run_global_search(
    constraints: &mut ConstraintStore,
    iterations: u32,
    initial_temperature: f32,
    cooling_factor: f32,
    number_of_parallel_searches: u32,
    random_seed: u32,
) -> Result<HillClimbResult, GlobalSearchError> {
    (0..number_of_parallel_searches)
        .into_par_iter()
        .inspect(|_| info!("Running global search"))
        .filter_map(|i| {
            let initial_schedule = Schedule::random(constraints, random_seed, Some(i));
            run_hill_climber(
                &mut constraints.clone(),
                initial_schedule,
                iterations,
                initial_temperature,
                cooling_factor,
            )
            .inspect_err(|error| {
                info!(
                    "Error occurred executing parallel run {:?}. Ignoring run",
                    i
                );
                error!("{:?}", error);
            })
            .ok()
        })
        .min_by_key(|(_, penalty, _)| *penalty)
        .inspect(|_| info!("Global search complete. Identified best schedule"))
        .ok_or(GlobalSearchError::NoResults)
}
