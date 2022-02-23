from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="map_generator",
    version="1.0",
    rust_extensions=[RustExtension("map_generator.map_generator", binding=Binding.PyO3)],
    zip_safe=False
)