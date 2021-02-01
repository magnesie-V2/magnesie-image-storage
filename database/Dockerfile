FROM mysql

# ENV MYSQL_RANDOM_ROOT_PASSWORD true
ENV MYSQL_ROOT_PASSWORD password

COPY *.sql /docker-entrypoint-initdb.d/
