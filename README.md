# Magnes.ie - Image Storage

## Build docker image
```sh
docker build --tag magnesie-image-storage .
```

## Run docker container
```sh
docker run -it -p 80:8000 -v /home/lpelecq/IMT/rust/files/:/hostedFiles magnesie-image-storage
```