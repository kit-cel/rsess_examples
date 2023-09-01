import subprocess
import json
import os

measurements_file = "measurements.json"
ess_configs = [# [e_max, n_max]
                 ["418", "50"],
                 ["796", "100"],
                 ["1560", "200"],
                 ["3072", "400"],
                 #["6096", "800"],
                 #["12136", "1600"],
                 #["24208", "3200"],
                 ]
dms = [["ESS"], ["OESS"]]
benches = [["./target/release/rsess-bench"],
           ["python3", "pyrsess-bench.py"]]

if not os.path.exists(measurements_file):
    with open(measurements_file, "x") as saveFile:
        json.dump([], saveFile)

with open(measurements_file, "r") as saveFile:
    measurements: list = json.load(saveFile)

for bench in benches:
    for dm in dms:
        for ess_config in ess_configs:
            command = bench + dm + ess_config
            ps = subprocess.run(command, capture_output=True)
            results = json.loads(ps.stdout)
            measurements.append(results)

with open(measurements_file, "w") as saveFile:
    json.dump(measurements, saveFile)
