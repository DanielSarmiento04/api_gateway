from . import app

@app.get('/users/home')
async def index():
    return {'text': 'Hello World! user api'}