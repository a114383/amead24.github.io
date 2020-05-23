# build the base operating system first
FROM debian

# attach metadata
LABEL maintainer=@tony

# ARG for defining environment variables 
# or you can pass to command line:
# DEBIAN_FRONTEND=noninteractive docker build -t some_image
ARG DEBIAN_FRONTEND=noninteractive

# install dependencies for linux setup
RUN apt-get update --fix-missing && \
    apt-get install -y \
    wget bzip2 ca-certificates \
    libglib2.0-0 libxext6 libsm6 libxrender1

# install dependencies for jupyter and miniconda
RUN echo 'export PATH=/opt/conda/bin:$PATH' > /etc/profile.d/conda.sh && \
    wget --quiet \
    https://repo.continuum.io/miniconda/Miniconda3-4.3.11-Linux-x86_64.sh -O \
    ~/miniconda.sh && \
    /bin/bash ~/miniconda.sh -b -p /opt/conda && \
    rm ~/miniconda.sh

# solving the zombie process problem
# that is, using tini will spawn a single process @ PID 1
# and when the first child exits so too will tini
RUN apt-get install -y curl grep sed dpkg && \
    TINI_VERSION=`curl https://github.com/krallin/tini/releases/latest |\
    grep -o "/v.*\"" | sed 's:^..\(.*\).$:\1:'` && \
    curl -L \
    "https://github.com/krallin/tini/releases/download/v${TINI_VERSION}/tini_${TINI_VERSION}.deb" > tini.deb && \
    dpkg -i tini.deb && \
    rm tini.deb && \
    apt-get clean

# define environment variables
ENV LANG=C.UTF-8 LC_ALL=C.UTF-8
ENV PATH /opt/conda/bin:$PATH

# entrypoint is for instructions specific upon instantiation
ENTRYPOINT [ "/usr/bin/tini", "--" ]

