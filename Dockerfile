FROM rust AS builder-server

RUN mkdir -p ~/.cargo \
  && echo '[source.crates-io]' > ~/.cargo/config.toml \
  && echo 'replace-with = "rsproxy-sparse"'  >> ~/.cargo/config.toml \
  && echo "[source.rsproxy]"  >> ~/.cargo/config.toml \
  && echo 'registry = "https://rsproxy.cn/crates.io-index"'   >> ~/.cargo/config.toml \
  && echo '[source.rsproxy-sparse]'   >> ~/.cargo/config.toml \
  && echo 'registry = "sparse+https://rsproxy.cn/index/"'  >> ~/.cargo/config.toml \
  && echo '[registries.rsproxy]'   >> ~/.cargo/config.toml \
  && echo 'index = "https://rsproxy.cn/crates.io-index"'   >> ~/.cargo/config.toml \
  && echo '[net]'   >> ~/.cargo/config.toml \
  && echo 'git-fetch-with-cli = true'   >> ~/.cargo/config.toml \
  && echo '' >> ~/.cargo/config.toml

WORKDIR /app

COPY Cargo.lock Cargo.toml ./
COPY server ./server

RUN cargo build --release

FROM debian:buster-slim

LABEL maintainer="linct96 linct96@outlook.com"
LABEL description="dst-server-admin"

WORKDIR /app
# 从上一阶段镜像中拷贝编译好的程序
COPY --from=builder-server /app/target/release/server ./

EXPOSE 9527

CMD ["server"] # 指定容器运行入口

# # 使用官方的 Node.js 镜像作为基础镜像
# FROM node:20-alpine as builder-web

# # 设置工作目录
# WORKDIR /app

# # 安装 pnpm
# RUN npm install -g pnpm --registry=https://npmreg.proxy.ustclug.org/

# # 复制 package.json 和 pnpm-lock.yaml 文件
# COPY .npmrc package.json pnpm-lock.yaml web/package.json ./

# # 安装项目依赖
# RUN pnpm install

# # 复制项目文件
# COPY . .

# # 构建 React 应用
# RUN pnpm run build:web

# # 使用 Nginx 作为 Web 服务器
# FROM nginx:alpine

# # 复制构建好的 React 应用到 Nginx 的默认目录
# COPY --from=builder-web /app/web/dist /usr/share/nginx/html

# # 暴露 80 端口
# EXPOSE 80

# # 启动 Nginx
# CMD ["nginx", "-g", "daemon off;"]
