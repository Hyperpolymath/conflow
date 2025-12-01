# Base configuration shared across environments
# Import and merge with environment-specific values

{
  application = {
    name = "my-app",
    version = "1.0.0",
  },

  defaults = {
    replicas = 1,
    log_level = "info",
    timeout_seconds = 30,
  },
}
