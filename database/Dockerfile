FROM mysql

# ENV MYSQL_RANDOM_ROOT_PASSWORD true
ENV MYSQL_ROOT_PASSWORD password
ENV MYSQL_TCP_PORT 3307

COPY *.sql /docker-entrypoint-initdb.d/
