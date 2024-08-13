FROM ubuntu:22.04

# Setting up java environment
RUN apt-get update && \
    apt-get install -y openjdk-11-jdk && \
    apt-get install -y ant && \
    apt-get clean

# Setting up rust environment
WORKDIR /bin/rust
RUN apt install curl build-essential -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >> instalation_script
RUN chmod +x instalation_script
RUN ./instalation_script -y
ENV PATH="/bin/rust/bin:$PATH"

WORKDIR /home