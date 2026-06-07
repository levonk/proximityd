#!/usr/bin/env bash
set -euo pipefail

IMAGE_NAME="proximityd:latest"
PLATFORMS="linux/amd64,linux/arm64"

case "${1:-}" in
    build)
        echo "Building $IMAGE_NAME for local platform..."
        docker build -t "$IMAGE_NAME" .
        ;;
    buildx)
        echo "Building $IMAGE_NAME for $PLATFORMS..."
        docker buildx build --platform "$PLATFORMS" -t "$IMAGE_NAME" .
        ;;
    buildx-push)
        echo "Building and pushing $IMAGE_NAME for $PLATFORMS..."
        docker buildx build --platform "$PLATFORMS" -t "$IMAGE_NAME" --push .
        ;;
    run)
        shift
        docker run --rm -it "$IMAGE_NAME" "$@"
        ;;
    compose-up)
        echo "Starting proximityd with docker-compose..."
        docker-compose up --build -d
        ;;
    compose-down)
        echo "Stopping proximityd..."
        docker-compose down
        ;;
    test)
        docker run --rm "$IMAGE_NAME" cargo test
        ;;
    *)
        cat <<EOF
Usage: $0 {build|buildx|buildx-push|run|compose-up|compose-down|test} [ARGS...]

  build        Build image for local platform
  buildx       Build multi-arch image (amd64 + arm64)
  buildx-push  Build and push multi-arch image
  run          Run container interactively
  compose-up   Start with docker-compose (daemon mode)
  compose-down Stop docker-compose services
  test         Run tests inside container
EOF
        exit 1
        ;;
esac
