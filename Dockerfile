FROM node:20-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY ./web /app
WORKDIR /app

#COPY ./web/package.json ./package.json
#COPY ./web/pnpm-lock.yaml ./pnpm-lock.yaml

RUN pnpm install
RUN pnpm run build

FROM nginx

COPY --from=base /app/dist /static

COPY nginx.conf /etc/nginx/nginx.conf