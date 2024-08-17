from setuptools import setup
from Cython.Build import cythonize

setup(
    ext_modules=cythonize("./fast_levenshtein/*.pyx", annotate=True, build_dir="build"),
    options={"build_ext": {"build_lib": "./fast_levenshtein"}},
)
