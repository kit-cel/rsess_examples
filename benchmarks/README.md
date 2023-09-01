# Benchmarks for RSESS / PyRSESS

## Usage

- Install PyRSESS and other Python packages (best use a virtual environment): `pip install numpy psutil pyrsess`
- Build the rsess-bench: `cargo build --release`
- Run the benchmarks: `python benchmark.py`
	- Optionally comment or uncomment the more demanding benchmarks
- Results are written to `measurements.json` as a list of JSON objects
