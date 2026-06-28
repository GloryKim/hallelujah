package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"sc-gin/internal/config"
	"sc-gin/internal/router"
)

func main() {
	cfg := config.Load()

	r := router.New()

	srv := &http.Server{
		Addr:    "127.0.0.1:" + cfg.Port,
		Handler: r,
	}

	go func() {
		log.Printf("[sc-gin] listening on 127.0.0.1:%s", cfg.Port)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("[sc-gin] listen error: %v", err)
		}
	}()

	// block until SIGTERM or SIGINT, then gracefully shut down
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGTERM, syscall.SIGINT)
	<-quit

	log.Println("[sc-gin] shutting down...")
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := srv.Shutdown(ctx); err != nil {
		log.Fatalf("[sc-gin] shutdown error: %v", err)
	}
	log.Println("[sc-gin] stopped")
}
