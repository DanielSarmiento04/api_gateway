from . import app

@app.get('/')
async def index():
    return {'text': 'Hello World! user api'}