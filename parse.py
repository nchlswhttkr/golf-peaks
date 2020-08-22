import sys
import yaml

slope_orientation_map = {
    'NW': 'down',
    'NE': 'left',
    'SE': 'up',
    'SW': 'right'
}
corner_orientation_map = {
    "NW": 'ne',
    "NE": 'se',
    "SE": 'sw',
    "SW": 'nw'
}


def main():
    # There's some stuff going on here with YAML tags that I haven't got a
    # clue about, skip the first three lines where they appear.
    # https://yaml.org/spec/1.1/#named%20tag%20handle/
    sys.stdin.readline()
    sys.stdin.readline()
    sys.stdin.readline()

    y = yaml.load(sys.stdin)

    starting_position = (0, 0)
    columns = y["MonoBehaviour"]["Level"].split("\n")
    for c in range(len(columns)):
        tiles = columns[c].replace('\r', '').split(';')
        for r in range(len(tiles)):
            tile = tiles[r].split(',')
            terrain = tile[0]
            elevation = tile[1] if len(tile) > 1 else 0
            if terrain == '0':
                print('ground,{},{},{}'.format(-c, -r, elevation))
                starting_position = (-c, -r)
            elif terrain == '1':
                print('hole,{},{},{}'.format(-c, -r, elevation))
            elif terrain == '2':
                print('ground,{},{},{}'.format(-c, -r, elevation))
            elif terrain == '6':
                print('ground,{},{},{},{}'.format(-c, -
                                                  r, elevation, corner_orientation_map[tile[2]]))
            elif terrain == '7':
                print('slope,{},{},{},{}'.format(-c, -r,
                                                 elevation, slope_orientation_map[tile[2]]))
    print()

    for card in y["MonoBehaviour"]["Cards"].split(";"):
        # FIXME terrible way of getting ground/air in the right order
        print(card[::-1])
    print()

    print("{},{}".format(starting_position[0], starting_position[1]))


if __name__ == "__main__":
    main()
