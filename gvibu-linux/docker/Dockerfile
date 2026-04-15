FROM alpine:latest
RUN apk add --no-cache bash
WORKDIR /gvibu
COPY vish /usr/local/bin/vish
RUN chmod +x /usr/local/bin/vish
CMD ["/bin/bash"]
