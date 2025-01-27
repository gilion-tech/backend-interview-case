FROM curlimages/curl:latest

RUN addgroup --system <group>
RUN adduser --system <user> --ingroup <group>
USER <user>:<group>

ENTRYPOINT ["/bin/sh", "-c", "curl -sf http://producer:3000/api/v1/data-stream | head -c 1024"]
# Implement your own
# service builder in this file
# if you want to leverage our
# docker-compose file to build
# and run
