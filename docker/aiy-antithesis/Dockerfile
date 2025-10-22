ARG AIY_TOOLS_IMAGE_TAG

FROM mysten/aiy-tools:$AIY_TOOLS_IMAGE_TAG AS setup

RUN apt update
RUN apt install python3 python3-pip -y

# copy configuration files to root
COPY ./new-genesis.sh /new-genesis.sh
COPY ./genesis /genesis

WORKDIR /

ARG AIY_NODE_A_TAG
ARG AIY_NODE_B_TAG
ARG AIY_PROTOCOL_CONFIG_CHAIN_OVERRIDE
ARG AIY_RUST_LOG
ARG STRESS_IMAGE_TAG
ENV TRANSACTION_DRIVER=$TRANSACTION_DRIVER

RUN ./new-genesis.sh
RUN echo "AIY_NODE_A_TAG=$AIY_NODE_A_TAG" >> /.env
RUN echo "AIY_NODE_B_TAG=$AIY_NODE_B_TAG" >> /.env
RUN echo "AIY_RUST_LOG=$AIY_RUST_LOG" >> /.env
RUN echo "AIY_PROTOCOL_CONFIG_CHAIN_OVERRIDE=$AIY_PROTOCOL_CONFIG_CHAIN_OVERRIDE" >> /.env
RUN echo "TRANSACTION_DRIVER=$TRANSACTION_DRIVER" >> /.env
RUN echo "STRESS_IMAGE_TAG=$STRESS_IMAGE_TAG" >> /.env

COPY ./docker-compose-antithesis.yaml /docker-compose.yaml

# when running an upgrade test, we required two docker files, one for before upgrade and one for after upgrade
# One of the files runs only the primary aiy-node image, and the other runs only the aiy-node-alt image.
RUN <<EOF
sed 's/aiy-node-alt:${AIY_NODE_B_TAG}/aiy-node:${AIY_NODE_A_TAG}/' < /docker-compose.yaml > docker-compose-1.yaml
sed 's/aiy-node:${AIY_NODE_A_TAG}/aiy-node-alt:${AIY_NODE_B_TAG}/' < /docker-compose.yaml > docker-compose-2.yaml
EOF

FROM scratch

COPY --from=setup /docker-compose-1.yaml /docker-compose-1.yaml
COPY --from=setup /docker-compose-2.yaml /docker-compose-2.yaml
COPY --from=setup /docker-compose.yaml /docker-compose.yaml

COPY /genesis/overlays/* /genesis/overlays/
COPY /genesis/static/* /genesis/static/
COPY --from=setup /genesis/files/* /genesis/files/
COPY --from=setup /.env /.env

