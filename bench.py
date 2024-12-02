#!/usr/bin/env python3

import os
import subprocess

FILENAME = "src/main.rs"

print("Size        AVX512/AVX2 (smaller is better)")
for S in (
    4,
    16,
    32,
    64,
    128,
    256,
    512,
    1 * 1024,
    2 * 1024,
    4 * 1024,
    8 * 1024,
    16 * 1024,
    32 * 1024,
    64 * 1024,
):
    lines = open(FILENAME).read().splitlines()
    with open(FILENAME, "w+") as w:
        for line in lines:
            if "const DATA_SIZE: usize" in line:
                line = f"const DATA_SIZE: usize = {S} * 1024;"
            w.write(line + "\n")

    avx2 = 0
    avx512 = 0
    N = 10
    for _ in range(N):
        r = subprocess.check_output(
            "cargo bench",
            env=os.environ | {"RUSTFLAGS": "-Ctarget-cpu=native"},
            shell=True,
            text=True,
            stderr=subprocess.DEVNULL,
        )
        lines = r.splitlines()
        assert "avx2" in lines[2]
        assert "avx512" in lines[3]
        avx2 += float(lines[2].split()[4].replace(",", ""))
        avx512 += float(lines[3].split()[4].replace(",", ""))
    # print(avx2 / N, avx512 / N)
    print(f"{S:5} KiB:     {avx512 / avx2:.3}")
