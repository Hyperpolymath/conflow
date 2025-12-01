// Configuration schema
package config

#Config: {
	// Application name
	name: string & =~"^[a-z][a-z0-9-]*$"

	// Number of replicas (1-10)
	replicas: int & >=1 & <=10

	// Port number
	port: int & >=1 & <=65535

	// Environment
	env: "dev" | "staging" | "prod"

	// Optional resource limits
	resources?: {
		cpu?:    string & =~"^[0-9]+(m|)$"
		memory?: string & =~"^[0-9]+(Mi|Gi)$"
	}
}
