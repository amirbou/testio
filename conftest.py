import pytest
import os

MY_DIR=os.path.dirname(__file__)

def pytest_addoption(parser):
    parser.addoption(
        "--verbose-fuse",
        action="store_true",
        default=False,
        help="Increase log verbosity of the testfs fuse"
    )
    parser.addoption(
        "--verbose-tester",
        action="store_true",
        default=False,
        help="Increase log verbosity of the low-level tester"
    )
    parser.addoption(
        "--fuse-bin",
        help="Path to the testfs fuse binary",
        default=os.path.join(MY_DIR, "target", "debug", "testio")
    )
    parser.addoption(
        "--tester-bin",
        help="Path to the tester binary",
        default=os.path.join(MY_DIR, "target", "debug", "tester")
    )
    parser.addoption(
        "--lib",
        help="Path to the library that will be tested",
        default=os.path.join(MY_DIR, "example", "libexample.so")
    )
