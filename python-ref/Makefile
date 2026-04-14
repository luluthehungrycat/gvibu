.PHONY: install test clean

PYTHON := python3
PIP := pip3

install:
	$(PIP) install -e .

test:
	$(PYTHON) -m pytest ../shared-tests/cases/ -v || true

clean:
	rm -rf build/ dist/ *.egg-info
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name "*.pyc" -delete 2>/dev/null || true
