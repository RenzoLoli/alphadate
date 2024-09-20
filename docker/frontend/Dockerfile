FROM node:lts-alpine

WORKDIR /usr/src/app

COPY ../../frontend/ .

RUN npm install

COPY . .

EXPOSE 4200

CMD [ "npm", "run", "start", "--" , "--host", "0.0.0.0" ]
