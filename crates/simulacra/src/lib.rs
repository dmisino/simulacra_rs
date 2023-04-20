#[derive(Debug)]
pub struct SimulacraError(pub String);

impl std::fmt::Display for SimulacraError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Simulacra error: {}", self.0)
  }
}