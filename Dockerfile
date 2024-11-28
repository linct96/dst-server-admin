# 构建 Web 服务器
FROM node:20-alpine AS builder-web

WORKDIR /app
# RUN npm install -g pnpm --registry=https://npmreg.proxy.ustclug.org/
RUN npm install -g pnpm
COPY .npmrc package.json pnpm-workspace.yaml pnpm-lock.yaml ./
COPY web/package.json ./web/package.json
RUN pnpm install
COPY web ./web
RUN pnpm run build:web

# # 使用 Nginx 作为 Web 服务器
# FROM nginx:alpine

# # 复制构建好的 React 应用到 Nginx 的默认目录
# COPY --from=builder-web /app/web/dist /usr/share/nginx/html

# # 暴露 80 端口
# EXPOSE 80

# # 启动 Nginx
# CMD ["nginx", "-g", "daemon off;"]

# 构建 Rust 服务端
FROM rust AS builder-server
# RUN mkdir -p ~/.cargo \
#   && echo '[source.crates-io]' > ~/.cargo/config.toml \
#   && echo 'replace-with = "rsproxy-sparse"'  >> ~/.cargo/config.toml \
#   && echo "[source.rsproxy]"  >> ~/.cargo/config.toml \
#   && echo 'registry = "https://rsproxy.cn/crates.io-index"'   >> ~/.cargo/config.toml \
#   && echo '[source.rsproxy-sparse]'   >> ~/.cargo/config.toml \
#   && echo 'registry = "sparse+https://rsproxy.cn/index/"'  >> ~/.cargo/config.toml \
#   && echo '[registries.rsproxy]'   >> ~/.cargo/config.toml \
#   && echo 'index = "https://rsproxy.cn/crates.io-index"'   >> ~/.cargo/config.toml \
#   && echo '[net]'   >> ~/.cargo/config.toml \
#   && echo 'git-fetch-with-cli = true'   >> ~/.cargo/config.toml \
#   && echo '' >> ~/.cargo/config.toml

WORKDIR /app
COPY Cargo.lock Cargo.toml ./
COPY server ./server
RUN cargo build --release

# 运行时镜像
FROM debian:buster-slim

LABEL maintainer="linct96 linct96@outlook.com"
LABEL description="dst-server-admin"

RUN sed -i 's/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list \
  && sed -i 's|security.debian.org/debian-security|mirrors.tuna.tsinghua.edu.cn/debian-security|g' /etc/apt/sources.list

# RUN sudo dpkg --add-architecture i386 \
#   && sudo apt-get -y update \
#   # && sudo apt-get -y dist-upgrade \
#   # && sudo apt-get -y install screen \
#   && echo 'lib install successfully'

WORKDIR /app
# 从上一阶段镜像中拷贝编译好的程序
COPY --from=builder-server /app/target/release/server ./
RUN chmod +x server
COPY --from=builder-web /app/web/dist ./dist

EXPOSE 9527
EXPOSE 10888/udp
EXPOSE 10998/udp
EXPOSE 10999/udp

CMD ["server"] # 指定容器运行入口


