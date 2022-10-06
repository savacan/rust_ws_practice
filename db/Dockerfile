FROM alpine AS builder


WORKDIR /app
RUN wget https://github.com/k0kubun/sqldef/releases/download/v0.11.38/mysqldef_linux_amd64.tar.gz
RUN tar -zxvf mysqldef_linux_amd64.tar.gz
RUN ls -l

FROM debian:bullseye-slim AS production

WORKDIR /app
COPY --from=builder /app/mysqldef /usr/bin/mysqldef

COPY ./ .

CMD [ "/app/migrate.sh" ]
