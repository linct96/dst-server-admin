# 使用官方的 Node.js 镜像作为基础镜像
FROM node:20-alpine

# 设置工作目录
WORKDIR /app

# 安装 pnpm
RUN npm install -g pnpm

# 复制 package.json 和 pnpm-lock.yaml 文件
COPY package.json pnpm-lock.yaml web/package.json ./

# 安装项目依赖
RUN pnpm install

# 复制项目文件
COPY . .

# 构建 React 应用
RUN pnpm run build:web

# 使用 Nginx 作为 Web 服务器
FROM nginx:alpine

# 复制构建好的 React 应用到 Nginx 的默认目录
COPY --from=0 /app/build/web/dist /usr/share/nginx/html

# 暴露 80 端口
EXPOSE 80

# 启动 Nginx
CMD ["nginx", "-g", "daemon off;"]