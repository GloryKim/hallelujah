package router

import (
	"github.com/gin-gonic/gin"
	"sc-gin/internal/handler"
)

func New() *gin.Engine {
	r := gin.Default()

	r.GET("/health", handler.Health)
	r.GET("/meta", handler.Meta)

	return r
}
