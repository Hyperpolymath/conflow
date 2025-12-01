# Staging environment configuration
let base = import "base.ncl" in

base & {
  environment = "staging",

  replicas = 2,

  log_level = "info",

  features = {
    debug_mode = false,
    hot_reload = false,
    profiling = true,
  },

  database = {
    host = "staging-db.internal",
    port = 5432,
    name = "myapp_staging",
  },
}
