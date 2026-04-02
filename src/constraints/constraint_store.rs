use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;

use crate::{constraints::Constraint, schedule::Schedule};
use rand::prelude::*;
use rand::rng;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize)]
/// Structure used for storing arbitrary constraints for optimisation
pub struct ConstraintStore {
    constraints: Vec<Constraint>,
}

impl ConstraintStore {
    /// Create a new empty constraint store
    pub fn new() -> Self {
        return ConstraintStore {
            constraints: Vec::new(),
        };
    }

    /// Add a new constraint to the store
    ///
    /// # Arguments
    /// * `constraints` - The constraints to store
    ///
    /// # Returns
    /// None
    pub fn push(&mut self, constraints: &mut Vec<Constraint>) {
        self.constraints.append(constraints);
    }

    /// Retrieves a constraint for optimisation from the store
    ///
    /// Randomly selects a constraint weighted by the penalties specified
    ///
    /// # Arguments
    /// `penalties` - The penalties incurred by the constraints
    ///
    /// # Returns
    /// * `&Constraint` - The constraint selected for optimisation
    /// * `None` - If no constraint was selected
    pub fn get_constraint_for_optimisation(
        &mut self,
        penalties: &HashMap<u32, u32>,
    ) -> Option<&Constraint> {
        self.constraints//.choose(&mut rng())
        .choose_weighted(&mut rng(), |c| {
            // NOTE: A base peanlty of 5 is added to each constraint to ensure even the
            // constraints that are incurring no penalty has a chance to be mutated
            (penalties.get(&c.id)
                .expect("Error: encountered inconsistent constraint ids between penalty calculation and constraint store")) + 5
        })
        .ok()
    }

    /// Finds a stored scheduled constraint that is compatible to be swapped with a given duration
    ///
    /// A higher chance is given to constraints with smaller duration to be selected if they are
    /// compatible to be swapped
    ///
    /// # Arguments
    /// * `constraint_id` - The id of the constraint for which we want to find a swappable constraint
    /// * `constraint_duration` - The duration of the constraint represented by `constraint_id`
    /// * `schedule` - The schedule
    ///
    /// # Returns
    /// * `&Constraint` - A constraint that is compatible to be swapped with
    /// * `None` - If no such constraint exist
    pub fn find_swappable_scheduled_constraint(
        &self,
        constraint_id: u32,
        constraint_duration: u8,
        schedule: &Schedule,
    ) -> Option<&Constraint> {
        let compatible_constriants: Vec<&Constraint> = self
            .constraints
            .iter()
            .filter(|c| {
                c.id != constraint_id
                    && schedule.is_constraint_scheduled(c.id)
                    && schedule.is_duration_free_or_owned_by(
                        &c.id,
                        constraint_duration,
                        schedule.get_scheduled_slot_for_constraint(c.id).unwrap(),
                    )
            })
            .collect();

        if compatible_constriants.is_empty() {
            return None;
        }

        Some(compatible_constriants.choose(&mut rng()).unwrap())
    }

    /// Retrives the stored constraint given its id
    ///
    /// # Arguments
    /// * `constraint_id` - The id of the constraint to fetch
    ///
    /// # Returns
    /// * `Constraint` - If a constraint with the specified id exist in the store
    /// * None - If no such constraint exists
    pub fn get_constraint(&self, constraint_id: u32) -> Option<&Constraint> {
        self.constraints.iter().find(|c| c.id == constraint_id)
    }

    /// Retrieves a list of stored constraint ids for a given `Constraint type`
    ///
    /// `Constraint type` is specified by the constraint's name
    ///
    /// # Arguments
    /// * `constraint_type` - The type of constraint (name)
    ///
    /// # Returns
    ///
    /// * `Vec<u32>` - A vector containing the list of ids for the specified type
    pub fn get_constraint_ids_of_type(&self, constraint_type: &String) -> Vec<u32> {
        self.constraints
            .iter()
            .filter(|c| c.name == *constraint_type)
            .map(|c| c.id)
            .collect()
    }

    /// Exports the constraints to a json file
    ///
    /// # Arguments
    /// * `file_name` - The name of the toml file containing the constraints
    pub fn export_constraints(&self, file_name: String) -> Result<(), Box<dyn Error>> {
        let json_string = serde_json::to_string_pretty(&self)?;
        std::fs::write(format!("{}.json", file_name), json_string)?;

        Ok(())
    }

    /// Prints metrics for the stored constraints under a specific schedule
    ///
    /// Metrics include,
    /// * Number/Names of scheduled constraints
    /// * Number/Names of non-scheduled constraints
    ///
    /// # Arguments
    /// * `schedule` - The schedule the constraints are scheduled under
    pub fn print_schedule_report(&self, schedule: &Schedule, total_incurred_penalty: u32) {
        let (scheduled, non_scheduled): (Vec<_>, Vec<_>) = self
            .constraints
            .iter()
            .partition(|c| schedule.is_constraint_scheduled(c.id));

        let total_constraints = self.constraints.len();
        let coverage =
            scheduled.len() as f32 / (scheduled.len() as f32 + non_scheduled.len() as f32) * 100.0;

        println!("\n{}", "=".repeat(40));
        println!("{:^40}", "SCHEDULE REPORT");
        println!("{}", "=".repeat(40));

        // Summary Statistics
        println!("{:<25} {:>14}", "Total Constraints:", total_constraints);
        println!("{:<25} {:>14}", "Successfully Scheduled:", scheduled.len());
        println!(
            "{:<25} {:>14}",
            "Non scheduled (Gaps):",
            non_scheduled.len()
        );
        println!("{:<25} {:>13.1}%", "Schedule Fulfillment:", coverage);
        println!(
            "{:<25} {:>14}",
            "Total incurred penalty:", total_incurred_penalty
        );

        println!("\n{}", "=".repeat(40));
        println!("Scheduled constraints");
        println!("{}", "=".repeat(40));
        scheduled
            .iter()
            .for_each(|c| println!("constraint_type: {}, id: {}", c.name, c.id));

        println!("\n{}", "=".repeat(40));
        println!("Non scheduled constraints");
        println!("{}", "=".repeat(40));
        non_scheduled
            .iter()
            .for_each(|c| println!("constraint_type: {}, id: {}", c.name, c.id));
        println!("{}", "-".repeat(40));
    }
}

impl<'a> IntoIterator for &'a mut ConstraintStore {
    type Item = &'a mut Constraint;
    type IntoIter = std::slice::IterMut<'a, Constraint>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.iter_mut()
    }
}

impl<'a> IntoIterator for &'a ConstraintStore {
    type Item = &'a Constraint;
    type IntoIter = std::slice::Iter<'a, Constraint>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.iter()
    }
}

impl fmt::Debug for ConstraintStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.constraints).finish()
    }
}

/// Reads stored constraint store configuration from json file, deserializes the constraints and
/// returns the store
///
/// # Arguments
/// * `file_name` - The name of the json file containing the serialized constraints store (excludes
/// the .json suffix)
///
/// # Returns
/// * `ConstraintStore` - The deserialized constraint store
/// * `io::Error` - If failed to open file with the given name
/// * `Error` - If cannot deserialize the contents of the json file into a constraint store
pub fn load_constraint_store_from_file(
    file_name: String,
) -> Result<ConstraintStore, Box<dyn Error>> {
    let json_reader = File::open(format!("{}.json", file_name))?;
    let constraints: ConstraintStore = serde_json::from_reader(json_reader)?;
    Ok(constraints)
}
