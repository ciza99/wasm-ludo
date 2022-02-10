####################################################################################################
## Build
####################################################################################################
FROM rust:latest AS builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk


WORKDIR /client
COPY ./ .
RUN trunk build --release
####################################################################################################
## Final image
####################################################################################################
FROM nginx:latest
ENV PORT=3000
COPY default.conf.template /etc/nginx/conf.d/default.conf.template
COPY --from=builder /client/dist /usr/share/nginx/html
CMD /bin/bash -c "envsubst '\$PORT' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf" && nginx -g 'daemon off;'
EXPOSE $PORT