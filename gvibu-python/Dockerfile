FROM python:3.11-slim
WORKDIR /gvibu
# Copy the Python vish surface and the GVIBU Python backend
COPY gvibu-python/vish.py /gvibu/vish.py
COPY gvibu-python/gvibu.py /gvibu/gvibu.py
COPY python-ref /gvibu/python-ref
ENV PYTHONPATH=/gvibu/python-ref
RUN chmod +x /gvibu/vish.py
RUN ln -s /gvibu/vish.py /usr/local/bin/vish
CMD ["python3", "/gvibu/vish.py"]
HEALTHCHECK --interval=5s --timeout=2s --start-period=1s CMD ["python3", "/gvibu/vish.py", "help"]
