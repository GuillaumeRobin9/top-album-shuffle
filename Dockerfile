FROM rust:latest as build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY . .
RUN cargo build --release --target wasm32-unknown-unknown

# Build the frontend
FROM node:18 AS frontend
WORKDIR /app
COPY pkg/package.json ./
RUN npm install
COPY pkg/ ./
COPY index.html ./
COPY style.css ./

# Nginx
FROM nginx:alpine
COPY --from=frontend /app/ /usr/share/nginx/html/

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]


