from asciimatics.screen import Screen

from map_generator.map_generator import generate_map_from_seed_with_steps


class IDIndex:
    index = {
        0: ' ',
        1: '█',
    }

    @staticmethod
    def get(key):
        if key in IDIndex.index:
            return IDIndex.index[key]
        else:
            return None

    @staticmethod
    def keys():
        return IDIndex.index.keys()


def main(screen):
    map_states = generate_map_from_seed_with_steps(screen.height, screen.width, 5)

    last_step = map_states[0]
    for (index, step) in enumerate(map_states):
        if last_step == step and index != 0:
            continue
        while True:
            for y in range(0, len(step)):
                for x in range(0, len(step[0])):
                    screen.print_at(IDIndex.get(step[y][x]), x, y)

            key = screen.get_key()
            if key == ord(' '):
                break
            screen.refresh()

        last_step = step

        if index == len(map_states) - 1:
            while True:
                key = screen.get_key()
                if key == ord('q'):
                    return


if __name__ == '__main__':
    Screen.wrapper(main)
