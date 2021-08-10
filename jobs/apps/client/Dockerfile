FROM node:16-slim
WORKDIR /var/www

COPY package.json yarn.lock ./
RUN yarn install

COPY . .
