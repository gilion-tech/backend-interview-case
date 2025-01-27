FROM curlimages/curl:latest
ENTRYPOINT ["/bin/sh", "-c", "curl -sf http://producer:3000/api/v1/data-stream | head -c 1024"]
# Implement your own
# service builder in this file
# if you want to leverage our
# docker-compose file to build
# and run
