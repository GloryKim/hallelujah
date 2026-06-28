package config

import "os"

type Config struct {
	Port string
}

func Load() *Config {
	port := os.Getenv("SIDECAR_PORT")
	if port == "" {
		port = "7101"
	}
	return &Config{Port: port}
}
