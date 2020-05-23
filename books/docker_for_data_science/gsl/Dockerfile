# Recommended to start with FROM for all but adv cases
# this will be the base public repo image which you build upon
# can be used: [FROM <image>, FROM <image>:<tag>, FROM <image>@<digest]
FROM gcc

# Contains meta-data in key:value pairs
LABEL maintainer=@tony

# Commands to execute once you've installed the base image
# the base image + these commands equal the new image
RUN apt-get update && \
    apt-get install -y \
    gsl-bin \
    libgsl0-dbg \
    libgsl0-dev

# stacking docker images ontop of each other
RUN apt-get update && \
    apt-get install -y gdb
