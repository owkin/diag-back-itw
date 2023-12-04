
# Backend for the Frontend Software Engineer Interview

## Overview

This is a simple backend written in Rust for the Frontend Software Engineer Interview.
It is a simple REST API that allows you to send Image processing request. It uses a InMemory database to store data & images.

## Run with the docker image

### Build the docker image

```bash
docker build --target production -t backend .
```

### Run the container

```bash
docker run -p 8080:80 backend
```

## Run locally (without docker)

Requirements:
- Rust 1.74.0

```bash
cargo run --release --bin backend -- --port 8080
```


## API

* `/job` (POST): Create a new image processing request -> Returns a job id [UUID v4]
  * Multipart fields
  * `image`: Image file (png, jpeg, tiff)
  * `filter`: Filter to apply on the image (Blurring, UnSharpening)
  * `sigma`: Sigma value for the filter
* `/job` (GET): Retrieves info about jobs
* `/job/:id` (GET): Retrieves info about a single job
* `/job/result/:id` (GET): Retrieves result image of a job

### Create a new image processing request

```bash
curl http://0.0.0.0:8080/job -F 'image=@/path/to/your/image' -F filter=Blurring -F sigma=5
```

### Get list of all jobs

```bash
curl http://0.0.0.0:8080/job
```


### Get a single job

```bash
curl http://0.0.0.0:8080/job/{job_id}
```

### Get a image result of a job

```bash
curl http://0.0.0.0:8080/job/result/{job_id}
```