# -----------
#    BUILD
# -----------
FROM node:22-alpine AS builder
WORKDIR /builder

COPY package*.json ./
RUN npm ci
COPY . .

RUN npm run prepare
RUN npm run build
RUN npm prune --omit=dev

# -----------
#   RUNTIME
# -----------
FROM node:22-alpine
WORKDIR /usr/bin/lesbin
COPY --from=builder /builder/build build/
COPY --from=builder /builder/node_modules node_modules/
COPY package.json .
EXPOSE 3000
ENV NODE_ENV=production
CMD [ "node", "build" ]

