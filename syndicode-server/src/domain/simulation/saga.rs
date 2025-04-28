use super::{game_state::GameState, ActionError};
use tracing;

// Renamed to reflect they include descriptions now
struct SagaStep<'a> {
    description: &'static str, // Added description field
    forward: ForwardAction<'a>,
    compensation: CompensateAction<'a>,
}

// Type alias for the forward action closure.
pub type ForwardAction<'a> = Box<dyn FnMut(&mut GameState) -> Result<(), ActionError> + 'a>;
// Type alias for the compensation action closure.
pub type CompensateAction<'a> = Box<dyn FnMut(&mut GameState) + 'a>;

/// Executes a sequence of actions (a saga) with rollback capabilities.
pub struct SagaExecutor<'a> {
    state: &'a mut GameState,
    // Store SagaStep structs which include the description
    steps: Vec<SagaStep<'a>>,
    // Store compensations along with their descriptions for better logging during rollback
    executed_compensations: Vec<(&'static str, CompensateAction<'a>)>, // Store description too
}

impl<'a> SagaExecutor<'a> {
    /// Creates a new SagaExecutor for the given mutable game state reference.
    pub fn new(state: &'a mut GameState) -> Self {
        SagaExecutor {
            state,
            steps: Vec::new(),
            executed_compensations: Vec::new(),
        }
    }

    /// Adds a step (including description) to the saga.
    /// Internal method called by the macro.
    /// Takes ownership of the closures via Box.
    pub fn add_step(
        &mut self,
        description: &'static str, // Added description parameter
        forward: ForwardAction<'a>,
        compensation: CompensateAction<'a>,
    ) {
        self.steps.push(SagaStep {
            description,
            forward,
            compensation,
        });
    }

    /// Executes the saga steps sequentially. Consumes the executor.
    pub fn execute(mut self) -> Result<(), ActionError> {
        for step in self.steps.drain(..) {
            // Deconstruct the step
            let mut forward_action = step.forward;
            let compensate_action = step.compensation;
            let description = step.description; // Get description for logging

            match forward_action(self.state) {
                Ok(_) => {
                    // Step succeeded. Log using the description.
                    tracing::debug!("Saga step '{}' executed successfully.", description);
                    // Store compensation with its description.
                    self.executed_compensations
                        .push((description, compensate_action));
                }
                Err(error) => {
                    // Step failed.
                    let completed_count = self.executed_compensations.len();
                    tracing::warn!(
                        error = %error, // Log the error from the failed step
                        failed_step = description, // Log which step failed
                        completed_steps = completed_count,
                        "Saga step '{}' failed. Rolling back {} previous step(s)...",
                        description,
                        completed_count
                    );

                    // Run compensations in reverse order.
                    for (comp_desc, mut compensation) in self.executed_compensations.drain(..).rev()
                    {
                        tracing::debug!("Executing compensation for step: '{}'...", comp_desc);
                        compensation(self.state);
                        tracing::debug!("Compensation for step '{}' finished.", comp_desc);
                    }
                    tracing::warn!("Saga rollback finished.");
                    // Return the original error that triggered the rollback
                    // Consider wrapping the error to include the step description?
                    // Example: Err(ActionError::SagaStepFailed{ description, source: Box::new(error) })
                    return Err(error); // Return original error for now
                }
            }
        }
        // All steps succeeded.
        tracing::debug!("Saga executed successfully (all steps completed).");
        Ok(())
    }
}

// --- saga_step! Macro ---

/// Macro to simplify adding steps (with descriptions) to a `SagaExecutor`.
///
/// **Usage:**
/// ```ignore
/// let mut executor = SagaExecutor::new(&mut game_state);
/// saga_step!(
///     executor, // The SagaExecutor instance (must be mut)
///     "Step Description", // A &'static str describing the step
///     |state| { /* forward logic returning Result<(), ActionError> */ Ok(()) },
///     |state| { /* compensation logic */ }
/// );
/// ```
#[macro_export]
macro_rules! saga_step {
    // Matcher: Executor identifier, Description string literal, Forward closure, Compensation closure
    ($executor:expr, $description:expr, $forward:expr, $compensate:expr) => {
        // The macro expands to code that creates the boxed closures and calls add_step.
        // Explicit types might be needed if inference fails, but usually okay.
        let forward_action: $crate::domain::simulation::saga::ForwardAction = Box::new($forward);
        let compensation_action: $crate::domain::simulation::saga::CompensateAction =
            Box::new($compensate);

        // Call the executor's method to add the step with description
        $executor.add_step($description, forward_action, compensation_action);
    };
}
