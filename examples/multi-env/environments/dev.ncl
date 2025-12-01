# Development environment configuration
let base = import "base.ncl" in

base & {
  environment = "dev",

  replicas = 1,

  log_level = "debug",

  features = {
    debug_mode = true,
    hot_reload = true,
    profiling = true,
  },

  database = {
    host = "localhost",
    port = 5432,
    name = "myapp_dev",
  },
}
