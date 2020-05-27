FROM node:latest

COPY ./index.js ./package.json ./yarn.lock ./

RUN yarn install

CMD ["node", "./index.js"]
