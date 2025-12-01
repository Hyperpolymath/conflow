// Application configuration schema
package config

#Config: {
	application: #Application
	services: [...#Service]
	database: #Database
	cache?:   #Cache
}

#Application: {
	name:        string & =~"^[a-z][a-z0-9-]*$"
	version:     string & =~"^[0-9]+\\.[0-9]+\\.[0-9]+$"
	environment: "dev" | "staging" | "prod"
}

#Service: {
	name:     string & =~"^[a-z][a-z0-9-]*$"
	port:     int & >=1 & <=65535
	replicas: int & >=1 & <=100
	health_check?: {
		path:     string
		interval: string & =~"^[0-9]+(s|m|h)$"
	}
}

#Database: {
	host:      string
	port:      int & >=1 & <=65535
	name:      string
	pool_size: int & >=1 & <=100
}

#Cache: {
	enabled: bool
	type:    "redis" | "memcached"
	host:    string
	port:    int & >=1 & <=65535
}
