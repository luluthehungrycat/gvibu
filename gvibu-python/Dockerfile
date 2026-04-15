FROM python:3.11-slim
WORKDIR /gvibu
# Copy the Python vish surface and the GVIBU Python backend
COPY gvibu-python/vish.py /gvibu/vish.py
COPY gvibu-python/gvibu.py /gvibu/gvibu.py
RUN chmod +x /gvibu/vish.py
RUN ln -s /gvibu/vish.py /usr/local/bin/vish
RUN chmod +x /gvibu/vish.py
CMD ["python3", "/gvibu/vish.py", "help"]
HEALTHCHECK --interval=5s --timeout=2s --start-period=1s CMD ["bash", "-lc", "vish gvibu help >/dev/null 2>&1 || true"]
