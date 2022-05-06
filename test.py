from concurrent.futures import process
from email.policy import default
import pytest
import subprocess
import tempfile
import os
import sys
import time
import random
import string


def create_rust_env(verbose):
    if verbose:
        return {"RUST_LOG": "debug"}
    else:
        return {}


@pytest.fixture(scope='session')
def fuse_env(request):
    return create_rust_env(request.config.getoption("--verbose-fuse"))


@pytest.fixture(scope='session')
def tester_env(request):
    return create_rust_env(request.config.getoption("--verbose-tester"))


@pytest.fixture(scope='session')
def fuse_bin(request):
    return request.config.getoption("--fuse-bin")


@pytest.fixture(scope='session')
def tester_bin(request):
    return request.config.getoption("--tester-bin")


@pytest.fixture(scope='session')
def lib(request):
    return request.config.getoption("--lib")

@pytest.fixture(scope='session')
def read_tester(tester_bin, tester_env, lib):
    def run_tester(full_path):
        return subprocess.run(
            [tester_bin, lib, full_path, "read"],
            env=tester_env,
            capture_output=True,
            check=True,
            timeout=30,
        )
    return run_tester


@pytest.fixture(scope='session')
def write_tester(tester_bin, tester_env, lib):
    def run_tester(full_path, data):
        return subprocess.run(
            [tester_bin, lib, full_path, "write", data],
            env=tester_env,
            capture_output=True,
            check=True,
            timeout=30,
        )
    return run_tester


@pytest.fixture(scope='session')
def fuse(fuse_bin, fuse_env):
    with tempfile.TemporaryDirectory() as tempdir:
        base_path = os.path.join(tempdir, "testfs")
        os.mkdir(base_path)
        fs = subprocess.Popen([fuse_bin, base_path], stderr=subprocess.PIPE, stdout=subprocess.PIPE, env=fuse_env)
        time.sleep(0.1)
        yield base_path
        subprocess.run(f'fusermount -u "{base_path}"', shell=True, check=True)
        fs.wait(timeout=3)
        out, err = fs.communicate()
        print(out)
        print(err, file=sys.stderr)


@pytest.mark.parametrize(
    "path",
    ["readempty", "readregular", "readone"] + [f"readX{i}" for i in range(2, 10)]
)
def test_read(fuse, read_tester, path):
    full_path = os.path.join(fuse, path)
    with open(full_path, 'rb') as reader:
        data = reader.read()
    
    test_data = read_tester(full_path)
    lines = test_data.stdout.splitlines()
    result = int(lines[-1].decode())
    extracted_test_data = b'\n'.join(lines[:-1])
    
    assert result == len(data)
    assert data == extracted_test_data


@pytest.mark.parametrize(
    "path",
    ["writeone"] + [f"writeX{i}" for i in range(2, 10)]
)
def test_write(fuse, write_tester, path):
    full_path = os.path.join(fuse, path)

    test_data = ''.join(random.choice(string.ascii_letters) for _ in range(10_000))

    result = write_tester(full_path, test_data)
    result = int(result.stdout.decode())
    
    with open(full_path, 'r') as reader:
        data = reader.read()

    # truncate the file    
    with open(full_path, 'w'):
        pass

    assert result == len(test_data)
    assert test_data == data
