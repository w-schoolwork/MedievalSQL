FROM debian

RUN apt-get update && apt-get install -y openssh-server git curl gcc && rm -rf /var/lib/apt/lists
RUN adduser user --uid 1000
RUN su user -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
COPY ./authorized_keys /home/user/.ssh/authorized_keys
ENTRYPOINT service ssh start && while true; do sleep 1000; done
