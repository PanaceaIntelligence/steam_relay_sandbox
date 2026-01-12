FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Steamworks SDK 빌드 및 런타임에 필요한 의존성 설치
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
       libclang-dev \
       cmake \
       pkg-config \
       libdbus-1-dev
