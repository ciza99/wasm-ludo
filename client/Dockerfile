FROM rust:latest as builder

COPY ./ ./

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

RUN trunk build --release

FROM ubuntu:latest

RUN apt update && apt install -y apache2

COPY --from=builder ./dist /var/www/html
COPY --from=builder ./.htaccess /var/www/html

RUN echo '. /etc/apache2/envvars' > /root/run_apache.sh && \
 echo 'mkdir -p /var/run/apache2' >> /root/run_apache.sh && \
 echo 'mkdir -p /var/lock/apache2' >> /root/run_apache.sh && \ 
 echo '/usr/sbin/apache2 -D FOREGROUND' >> /root/run_apache.sh && \ 
 chmod 755 /root/run_apache.sh

EXPOSE 80

CMD /root/run_apache.sh
