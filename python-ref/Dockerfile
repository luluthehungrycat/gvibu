FROM python:3.11-slim

WORKDIR /app

COPY pyproject.toml ./

RUN pip install --no-cache-dir -e .

ENV PATH="/app:${PATH}"

CMD ["python", "-m", "gvibu_ref.main"]
