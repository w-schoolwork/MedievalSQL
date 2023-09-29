FROM ubuntu

RUN apt-get update && apt-get install -y openssh-server git curl gcc pkg-config libssl-dev && rm -rf /var/lib/apt/lists
RUN adduser user --uid 1000
RUN su user -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
RUN su user -c "source ~/.cargo/env && cargo install sqlx-cli"
RUN echo '\nDATABASE_URL=postgres://postgres:postgres@db/msql\n' >> /etc/environment
COPY ./authorized_keys /home/user/.ssh/authorized_keys
RUN chown -R user /home/user/.ssh
ENTRYPOINT service ssh start && while true; do sleep 1000; done
