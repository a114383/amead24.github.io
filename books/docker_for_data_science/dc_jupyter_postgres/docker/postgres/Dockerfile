FROM postgres:alpine
# while loading up the base image  we copy two files from
# the context to the image.
#
# the copy location is a image hook point so the files inside
# this directory /*-entrypoint-* will get executed on build
COPY get_data.sh /docker-entrypoint-initdb.d/get_data.sh
COPY initdb.sql /docker-entrypoint-initdb.d/initdb.sql
