# Build image
FROM node:20-slim as build

RUN corepack enable

WORKDIR /app
COPY . .
RUN pnpm install --frozen-lockfile
RUN pnpm run build


# Runtime image
FROM node:20.9.0-alpine3.18

WORKDIR /www/app

RUN apk add curl

COPY --from=build /app/node_modules node_modules
COPY --from=build /app/build/ build
COPY package.json .
COPY .env.production .env.production

EXPOSE 3000

ENV WEBSITE_PROTOCOL_HEADER="X-Forwarded-Proto"
ENV WEBSITE_HOST_HEADER="X-Forwarded-Host"

CMD ["node", "build"]
