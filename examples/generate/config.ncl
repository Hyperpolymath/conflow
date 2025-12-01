# Nickel configuration generator
# Demonstrates using functions for DRY configuration

let environment = "dev" in
let base_replicas = 1 in

# Helper function to compute replicas based on environment
let compute_replicas = fun env =>
  if env == "prod" then 5
  else if env == "staging" then 3
  else base_replicas
in

# Main configuration
{
  name = "my-app",

  replicas = compute_replicas environment,

  port = 8080,

  env = environment,

  resources = {
    cpu =
      if environment == "prod" then "500m"
      else "100m",
    memory =
      if environment == "prod" then "512Mi"
      else "256Mi",
  },
}
