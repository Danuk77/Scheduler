use sobol_burley::sample;

/// Generates a quasi-random permutation of constraint indices using a Sobol sequence.
///
/// Uses a Fisher-Yates shuffle driven by Sobol sampling to produce an ordering
/// of constraint indices.
///
/// # Arguments
///
/// * `number_of_constraints` - The number of constraints to generate indices for.
/// * `random_seed` - Scramble seed for the Sobol sequence.
/// * `schedule_index` - Which point in the Sobol sequence to use. If generating multiple
/// sequences, this index will ensure the sequences are distant from each other
///
/// # Returns
/// * Vec<usize> - A permutation of the numbers from 0 to number_of_constraints
pub fn generate_sobol_indices(
    number_of_constraints: usize,
    random_seed: u32,
    schedule_index: u32,
) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..number_of_constraints).collect();

    for i in 0..number_of_constraints - 1 {
        let sobol_val = sample(schedule_index, i as u32, random_seed);
        let j = i + (sobol_val * (number_of_constraints - 1) as f32) as usize;
        let j = j.min(number_of_constraints - 1);
        indices.swap(i, j);
    }

    indices
}
