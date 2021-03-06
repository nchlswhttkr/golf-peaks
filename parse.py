import sys

slope_orientation_map = {
    'NW': 'south',
    'NE': 'west',
    'SE': 'north',
    'SW': 'east'
}
corner_orientation_map = {
    "NW": 'ne',
    "NE": 'se',
    "SE": 'sw',
    "SW": 'nw'
}

conveyor_orientation_map = {
    "NW": 'north',
    "NE": 'east',
    "SE": 'south',
    "SW": 'west'
}


def main():
    level = ''
    cards = ''

    # Read up until the level starts
    line = sys.stdin.readline()
    while not line.startswith('  Level:'):
        line = sys.stdin.readline()

    # The level may span many lines, it stops right before the cards
    allowed_characters = '0123456789-,;NESW\\rn\n'
    while not line.startswith('  Cards:'):
        for c in line:
            if c in allowed_characters:
                level += c
        line = sys.stdin.readline()

    cards += line.lstrip('  Cards: ').rstrip()
    level = level.replace('\\n', '\n')   # interpret newlines
    level = level.replace('\\r', '')     # strip carriage returns
    level = level.replace('\n\n', '\n')  # some levels have duplicated newlines

    starting_position = None
    portals = {}
    columns = level.split("\n")
    for c in range(len(columns)):
        tiles = columns[c].rstrip().split(';')
        for r in range(len(tiles)):
            tile = tiles[r].split(',')
            terrain = tile[0]
            elevation = tile[1] if len(tile) > 1 else 0
            if terrain == '0':
                print('ground,{},{},{}'.format(-c, -r, elevation))
                # one level has two golf tiles? the first is the correct start
                if starting_position is None:
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
                                                 elevation, slope_orientation_map[tile[2] if len(tile) > 2 else 'NW']))
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

    for card in cards.split(";"):
        print(card)
    print()

    print("{},{}".format(starting_position[0], starting_position[1]))


if __name__ == "__main__":
    main()
