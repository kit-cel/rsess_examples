from pyrsess import ESS, OESS
import datetime
import time
import numpy as np
import sys
import psutil
import os
import json

# consts
SHAPING_RATE = 1.5
ASK = 8
NUM_TRANSMISSIONS = 10000
NUM_REPETITIONS = 10

process = psutil.Process(os.getpid())

def benchmark(dm_class, e_max, n_max) -> (float, float, float):
    """ Returns trellis build time, encoding time, decoding time """

    resident1_kB = process.memory_info().rss / 1024
    start = datetime.datetime.now()
    dm = dm_class(e_max, n_max, ASK)
    end = datetime.datetime.now()
    resident2_kB = process.memory_info().rss / 1024

    build_time_ms = (end - start) / datetime.timedelta(milliseconds=1)

    data = np.random.randint(0, 2, (NUM_TRANSMISSIONS, dm.num_data_bits()))

    start = datetime.datetime.now()
    val = dm.multi_encode(data)
    end = datetime.datetime.now()
    encode_time_ms = (end - start) / datetime.timedelta(milliseconds=1)

    start = datetime.datetime.now()
    dm.multi_decode(val)
    end = datetime.datetime.now()
    decode_time_ms = (end - start) / datetime.timedelta(milliseconds=1)

    results = {
        "tool": "prsess-bench",
        "n_max": dm.n_max(),
        "e_max": dm.e_max(),
        "r_sh": dm.num_data_bits() / dm.n_max(),
        "class": str(type(dm)),
        "resident_MB": resident2_kB / 1024,
        "diff_resident_kB": resident2_kB - resident1_kB,
        "trellis_build_time_ms": build_time_ms,
        "encoding_time_ms": encode_time_ms,
        "decoding_time_ms": decode_time_ms,
        "time_stamp": datetime.datetime.now().strftime("%d.%m.%Y %H:%M:%S"),
    }

    return results

if __name__ == '__main__':
    args = sys.argv

    dm_class = OESS if args[1] == "OESS" else ESS
    e_max = int(args[2])
    n_max = int(args[3])

    results = benchmark(dm_class, e_max, n_max)

    print(json.dumps(results))
