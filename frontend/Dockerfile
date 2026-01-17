# Build stage
FROM node:20-alpine as builder

ARG NUXT_PUBLIC_APP_PORT=3000
ARG NUXT_PUBLIC_ENABLE_URL_AUTH=false

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source code
COPY . .

# Build the application
RUN npm run build

# Runtime stage
FROM node:20-alpine

WORKDIR /app

# Copy built application from builder
COPY --from=builder /app/.output ./

# Expose the port
EXPOSE 3700

# Run the application
CMD ["node", "server/index.mjs"]
