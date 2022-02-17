# stage1 - build react app first
FROM node:lts as build
WORKDIR /app

ARG api_url=http://localhost:8000

ENV REACT_APP_GRAPHQL_URL $api_url
ENV PATH /app/node_modules/.bin:$PATH

COPY ./package.json /app/
COPY ./package-lock.json /app/
RUN npm install

COPY . /app
RUN npm run build

# stage 2 - build the final image and copy the react build files
FROM nginx:latest
COPY --from=build /app/build /usr/share/nginx/html
CMD ["nginx-debug", "-g", "daemon off;"]
