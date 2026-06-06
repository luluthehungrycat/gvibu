FROM python:3.11-slim

WORKDIR /app

COPY python-ref/pyproject.toml ./
COPY python-ref/gvibu_ref/ ./gvibu_ref/

RUN pip install --no-cache-dir -e .

ENV PATH="/app:${PATH}"

CMD ["python", "-m", "gvibu_ref.main"]
