import os, sys

from map_generator import generate_with_rooms

symbols = {
    0: " ",
    1: "#",
    2: "%"
}

seed = int.from_bytes(os.urandom(8), 'big')
world = generate_with_rooms(32, 96, seed)
for row in world:
    for cell in row:
        sys.stdout.write(symbols[cell])
    sys.stdout.write("\n")