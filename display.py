import map_generator.map_generator as map_generator

id_index = {
            -1: '╳',
            0: ' ',
            1: '█',
            }

world = map_generator.generate_map(32, 96)

for row in world:
    for cell in row:
        if cell in id_index:
            print(id_index[cell], end='')
        else:
            print(id_index[-1], end='')
    print('')
