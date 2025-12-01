// Pipeline schema for conflow
// This CUE schema validates .conflow.yaml files

package conflow

#Pipeline: {
	// Pipeline version
	version: "1" | *"1"

	// Pipeline name
	name: string & =~"^[a-z][a-z0-9-]*$"

	// Optional description
	description?: string

	// Pipeline stages
	stages: [...#Stage]

	// Global environment variables
	env?: [string]: string

	// Cache configuration
	cache?: #CacheConfig
}

#Stage: {
	// Stage name (unique within pipeline)
	name: string & =~"^[a-z][a-z0-9-]*$"

	// Optional description
	description?: string

	// Tool configuration
	tool: #CueTool | #NickelTool | #ShellTool

	// Input specification
	input: #Input

	// Output specification
	output?: #Output

	// Dependencies (other stage names)
	depends_on?: [...string]

	// Continue on failure
	allow_failure?: bool | *false

	// Stage environment variables
	env?: [string]: string

	// Run condition
	condition?: #StageCondition
}

// CUE tool configuration
#CueTool: {
	type:       "cue"
	command:    "vet" | "export" | "eval" | "fmt" | "def"
	schemas?:   [...string]
	flags?:     [...string]
	out_format?: "json" | "yaml" | "toml" | "cue"
}

// Nickel tool configuration
#NickelTool: {
	type:    "nickel"
	command: "export" | "typecheck" | "query" | "format"
	file?:   string
	flags?:  [...string]
	format?: "json" | "yaml" | "toml"
}

// Shell tool configuration
#ShellTool: {
	type:    "shell"
	command: string & !=""
	shell?:  string | *"bash"
}

// Input specification
#Input: string | [...string] | {from_stage: string}

// Output specification
#Output: string | {
	path:   string
	format: "json" | "yaml" | "toml" | "cue" | "text"
}

// Cache configuration
#CacheConfig: {
	enabled?:      bool | *true
	directory?:    string | *".conflow/cache"
	invalidation?: "content_hash" | "mtime" | "manual" | *"content_hash"
}

// Stage conditions
#StageCondition: "always" | "never" | {file_exists: string} | {env_set: string} | {env_equals: {
	var:   string
	value: string
}}
