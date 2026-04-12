use log::info;

/// A simple struct to store simple statistics about a run of the optimisation algorithm
#[derive(Default)]
pub struct OptimisationStats {
    pub move_count: u32,
    pub schedule_count: u32,
    pub unscheduling_scheduled_count: u32,
    pub unscheduling_unscheduled_count: u32,
    pub revert_count: u32,
    pub reset_count: u32,
}

impl OptimisationStats {
    /// Logs a report of the optimisation algorithms statistics to standard out
    ///
    /// NOTE: This code snippet is AI generated
    pub fn generate_optimisation_report(&self) {
        let total_iterations = self.move_count
            + self.schedule_count
            + self.unscheduling_scheduled_count
            + self.unscheduling_unscheduled_count;

        info!("─── OPTIMISATION STATS ───");

        info!("▶ Throughput");
        info!("  ├─ Total Iterations: {}", total_iterations);
        info!("  ├─ Total resets: {}", self.reset_count);
        info!("  ├─ Total reverts: {}", self.revert_count);

        info!("▶ Mutation Types");
        info!("  ├─ Moves: {}", self.move_count);
        info!("  ├─ New schedulings: {}", self.schedule_count);
        info!(
            "  └─ Mass unscheduling scheduled: {}",
            self.unscheduling_scheduled_count
        );
        info!(
            "  └─ Mass unscheduling unscheduled: {}",
            self.unscheduling_unscheduled_count
        );
        info!("──────────────────────────");
    }
}
