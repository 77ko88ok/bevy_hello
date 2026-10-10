FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# 1. Bevyのネイティブビルドやオーディオ・ウィンドウ制御に必要なシステム依存関係をインストール
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libx11-dev \
        libasound2-dev \
        libudev-dev \
        libwayland-dev \
        libxkbcommon-dev \
        lld \
        clang \
    && rm -rf /var/lib/apt/lists/*

# 2. 一般ユーザー（vscode）に切り替えてRustのWASMターゲットとTrunkを追加
USER vscode
RUN rustup target add wasm32-unknown-unknown

# Trunkのインストール（ビルドツール）
RUN cargo install trunk
