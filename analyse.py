import csv
import argparse
import matplotlib.pyplot as plt


def plot_data(stats, total_stats, num_players):
    hand_names = []
    win_rates = []
    iterations = total_stats[0]
    num_hands = total_stats[1]
    for hand in stats:
        hand_names.append(hand[0])
        win_rates.append(hand[1])
    reversed_hand_names = hand_names[::-1]
    reversed_win_rates = win_rates[::-1]
    plt.barh(reversed_hand_names, reversed_win_rates)
    plt.xlabel("Win Rate")
    plt.title(
        f"Win Rates for Each Hand with {num_players} Players After {iterations} Iterations. Total Hands: {num_hands}")

    for i in range(len(reversed_win_rates)):
        plt.text(reversed_win_rates[i], reversed_hand_names[i], str(
            reversed_win_rates[i]))

    plt.show()


def read_file(file_name):
    with open(file_name) as csv_file:
        csv_reader = csv.reader(csv_file)
        line_count = 0
        hands = []
        total_stats = []
        for row in csv_reader:
            line_contents = row[0].split(",")
            if line_count == 0:
                iterations = line_contents[0]
                num_hands = line_contents[1]
                total_stats.append(iterations)
                total_stats.append(num_hands)
            else:
                hand_name = line_contents[0]
                num_times_played = int(line_contents[1])
                num_times_won = int(line_contents[2])

                win_ratio = 0 if num_times_played == 0 else num_times_won/num_times_played
                hands.append((hand_name, win_ratio))

            line_count += 1

        return (sorted(hands, key=lambda x: x[1], reverse=True), total_stats)


def get_args():
    parser = argparse.ArgumentParser(description='Process some integers.')
    parser.add_argument('num_players', metavar='N', type=int, nargs='?', default=8,
                        help='Number of players at a table (default: 8)')
    parser.add_argument('-v', '--verbose', action='store_true',
                        help='Increase detail in hand outputs.')

    args = parser.parse_args()

    return args


if __name__ == "__main__":
    args = get_args()
    num_players = args.num_players
    verbose = args.verbose
    verbose_s = "_verbose" if verbose else ""

    (stats, total_stats) = read_file(
        f"output_{num_players}_players{verbose_s}.csv")

    for hand in stats:
        print(hand)

    plot_data(stats, total_stats, num_players)
