// Validation schema for generated config
package config

#Config: {
	name:     string
	replicas: int & >=1 & <=10
	port:     int & >=1 & <=65535
	env:      "dev" | "staging" | "prod"
	resources: {
		cpu:    string
		memory: string
	}
}
