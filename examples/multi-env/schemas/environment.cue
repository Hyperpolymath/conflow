// Environment configuration schema
package environment

#EnvironmentConfig: {
	application: {
		name:    string
		version: string & =~"^[0-9]+\\.[0-9]+\\.[0-9]+$"
	}

	defaults?: {
		replicas?:        int & >=1
		log_level?:       #LogLevel
		timeout_seconds?: int & >=1
	}

	environment: "dev" | "staging" | "prod"

	replicas: int & >=1 & <=100

	log_level: #LogLevel

	features: {
		debug_mode:  bool
		hot_reload:  bool
		profiling:   bool
	}

	database: {
		host: string
		port: int & >=1 & <=65535
		name: string
	}
}

#LogLevel: "debug" | "info" | "warn" | "error"
