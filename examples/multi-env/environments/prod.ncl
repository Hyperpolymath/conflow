# Production environment configuration
let base = import "base.ncl" in

base & {
  environment = "prod",

  replicas = 5,

  log_level = "warn",

  features = {
    debug_mode = false,
    hot_reload = false,
    profiling = false,
  },

  database = {
    host = "prod-db.internal",
    port = 5432,
    name = "myapp_prod",
  },
}
