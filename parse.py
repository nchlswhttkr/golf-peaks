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

conveyor_orientation_map = {
    "NW": 'up',
    "NE": 'right',
    "SE": 'down',
    "SW": 'left'
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
    portals = {}
    columns = y["MonoBehaviour"]["Level"].split("\n")
    for c in range(len(columns)):
        tiles = columns[c].replace('\r', '').rstrip().split(';')
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
            elif terrain == '3':
                print('water,{},{},{}'.format(-c, -r, elevation))
            elif terrain == '4':
                print('trap,{},{},{}'.format(-c, -r, elevation))
            elif terrain == '6':
                print('ground,{},{},{},{}'.format(-c, -
                                                  r, elevation, corner_orientation_map[tile[2] if len(tile) > 2 else 'NW']))
            elif terrain == '7':
                print('slope,{},{},{},{}'.format(-c, -r,
                                                 elevation, slope_orientation_map[tile[2]]))
            elif terrain == '8':
                pass  # OOB tile, skip
            elif terrain == '10':
                print('spring,{},{},{}'.format(-c, -r, elevation))
            elif terrain == '12':
                print('sand,{},{},{}'.format(-c, -r, elevation))
            elif terrain == '13':
                pair_number = tile[3] if len(tile) > 3 else -1
                if pair_number in portals:
                    partner = portals[pair_number]
                    print('portal,{},{},{},{},{}'.format(-c, -
                                                         r, elevation, partner[0], partner[1]))
                    print('portal,{},{},{},{},{}'.format(
                        partner[0], partner[1], partner[2], -c, -r))
                else:
                    portals[pair_number] = (-c, -r, elevation)
            elif terrain == '14':
                print('ice,{},{},{}'.format(-c, -r, elevation))
            elif terrain == '15':
                print("conveyor,{},{},{},{}".format(-c, -r,
                                                    elevation, conveyor_orientation_map[tile[2] if len(tile) > 2 else 'NW']))
            elif terrain == '17':
                print('ice,{},{},{},{}'.format(-c, -r, elevation,
                                               corner_orientation_map[tile[2] if len(tile) > 2 else 'NW']))
    print()

    for card in y["MonoBehaviour"]["Cards"].split(";"):
        # FIXME terrible way of getting ground/air in the right order
        print(card[::-1])
    print()

    print("{},{}".format(starting_position[0], starting_position[1]))


if __name__ == "__main__":
    main()
