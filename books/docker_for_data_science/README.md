## docker_book
doing some book learning on docker

flags:
-----
  -e pass environment variables to container
     ex: -e GRANT_SUDO=yes --user root

  -d detached mode, running a container as daemon
  -p port <localport>:<externalport>

  -i interactive contaner
  -t TTY commands, tie container to CLI w/ stdio

  -v location of volume to mount, can be local or docker volume

1. Docker containers lack persistance, and thus you can link them to docker volumes for databases.
    ```
    docker volume create --name redis-dbstore
    docker run -d --name this_redis -v redis-dbstore:/data redis
    docker exec -it this_redis redis
    ```

2. Setting up communication between containers, redis_volume <=> redis_container <=> jupyter

3. Using docker-compose you can build deployable software suites, or container infrastructure.  See `jupyter_redis/` for configuration files and builds.
   ```
   docker-compose up    #start setup using docker-compose.yml
   docker-compose down  #teardown the running network
   ```

4. However when using your own configuration files, or you've made changes to a local docker image in cache you need to first run `docker-compose build` in order to refresh any changes.
   ```
   docker-compose build  # will collect changes and build the image for the cache
   docker-compose up     # just executes the image that matches in cache
   ```

5. `dc_network/` is an example of using multiple images over a shared network. Once running you can access each image individually using their service name.
