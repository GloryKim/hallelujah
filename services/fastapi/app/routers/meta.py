from fastapi import APIRouter

router = APIRouter()


@router.get("/meta")
async def meta():
    return {"service": "fastapi", "version": "0.1.0"}
