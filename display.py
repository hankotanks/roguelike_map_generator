from asciimatics.screen import Screen

from map_generator.map_generator import generate_from_seed_with_steps


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
    map_states = generate_from_seed_with_steps(screen.height - 1, screen.width, 5)

    pruned_map_states = {}

    for key, value in map_states.items():
        if value not in pruned_map_states.values():
            pruned_map_states[key] = value

    map_states = pruned_map_states

    for (index, key) in enumerate(list(sorted(map_states.keys()))):
        step = map_states[key]

        screen.print_at(key.ljust(screen.width, '█'), 0, 0)

        while True:
            for y in range(0, len(step)):
                for x in range(0, len(step[0])):
                    screen.print_at(IDIndex.get(step[y][x]), x, y + 1)

            key_press = screen.get_key()
            if key_press == ord(' '):
                break

            screen.refresh()

        if key == list(sorted(map_states.keys()))[-1]:
            while True:
                key_press = screen.get_key()
                if key_press == ord('q'):
                    return


if __name__ == '__main__':
    Screen.wrapper(main)
