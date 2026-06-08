use core::fmt;

use log::{error, info};
use rayon::prelude::*;

use crate::{
    config::{OptimisationStrategyConfig, PenaltiesConfig}, constraints::constraint_store::ConstraintStore,
    hill_climber::run_hill_climber, schedule::Schedule, stats::OptimisationStats,
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

/// Runs a global search algorithm to find the optimum schedule for the specified constraints
///
/// Runs a specified number of local searches in parallel and returns the best result
/// Each local search starts with an initial schedule that is seeded using quasi randomly
///
/// Each local search is a simulated annealing algorithm with restarts upon reaching local minima
///
/// # Arguments
/// * `constraints` (&mut ConstraintStore) - The constraint store containing the constraints to
/// schedule
/// * `iterations` (u32) - The number of iteration to run each local search for
/// * `initial_temperature` (f32) - The starting temperature of each local search
/// * `cooling_factore` (f32) - The cooling temperature for each local search
/// * `number_of_parallel_searches` (u32) - The number of parallel local searches to run
/// * `random_seed` (u32) - The random seed used for the quasi random generator
/// * `penalties_config (&PenaltiesConfig)` - The configuration specifying values used for penalties
///     during optimisation
/// * `optimisation_strategy_config (OptimisationStrategyConfig)` - The configuration speciifying
///     chances used in choosing optimisation strategy
///
/// # Returns
/// * (Schedule, u32, OptimisationStats) - The idenfied best (schedule, its penalty,
/// optimisation_statistics)
/// * GlobalSearchError - Error if it was not possible to successfully complete a single local
/// search
pub fn run_global_search(
    constraints: &mut ConstraintStore,
    iterations: u32,
    initial_temperature: f32,
    cooling_factor: f32,
    number_of_parallel_searches: u32,
    random_seed: u32,
    penalties_config: &PenaltiesConfig,
    optimisation_strategy_config: &OptimisationStrategyConfig
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
                penalties_config,
                optimisation_strategy_config
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
