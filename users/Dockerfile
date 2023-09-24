# Select the base image
FROM  python:3.10-alpine

# Set the working directory
WORKDIR /code

# Copy the requirements file
COPY ./requirements.txt /code/requirements.txt

# Add OpenCV dependencies
RUN apt-get update

# Install the dependencies
RUN pip install --no-cache-dir --upgrade -r /code/requirements.txt

# Copy the source code
COPY ./app /code/app

# Copy the main.py file
COPY ./main.py /code/main.py

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]