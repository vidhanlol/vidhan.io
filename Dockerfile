FROM node:17.1.0-alpine

WORKDIR /app
COPY . .

RUN yarn install --prod
RUN yarn add --dev typescript
RUN yarn build
CMD [ "yarn", "start" ]