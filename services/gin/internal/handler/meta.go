package handler

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func Meta(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"service": "gin",
		"version": "0.1.0",
	})
}
