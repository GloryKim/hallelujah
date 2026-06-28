import os
import signal
import uvicorn
from contextlib import asynccontextmanager
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from app.routers import health, meta


@asynccontextmanager
async def lifespan(app: FastAPI):
    print("[sc-fastapi] starting up")
    yield
    print("[sc-fastapi] shutting down")


app = FastAPI(title="sc-fastapi", version="0.1.0", lifespan=lifespan)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)
app.include_router(health.router)
app.include_router(meta.router)


if __name__ == "__main__":
    port = int(os.environ.get("SIDECAR_PORT", 7103))
    uvicorn.run("app.main:app", host="127.0.0.1", port=port)
