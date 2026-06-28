package router

import (
	"github.com/gin-gonic/gin"
	"sc-gin/internal/handler"
	"sc-gin/internal/middleware"
)

func New() *gin.Engine {
	r := gin.Default()
	r.Use(middleware.CORS())

	r.GET("/health", handler.Health)
	r.GET("/meta", handler.Meta)

	return r
}
