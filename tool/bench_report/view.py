import matplotlib.pyplot as plt
import os

REPORTS_DIRECTORY = "reports"


def main():
    for name in os.listdir(REPORTS_DIRECTORY):
        path = os.path.join(REPORTS_DIRECTORY, name)

        if os.path.isfile(path):
            thresholds = {
                240: False,
                120: False,
                60: False,
                30: False,
            }

            data_x = []
            data_y = []

            with open(path) as file:
                for line in file:
                    line = line.strip()  # remove new line character

                    if line:
                        spawned, fps = line.split(",")
                        spawned = int(spawned)
                        fps = int(fps)

                        for threshold, is_active in thresholds.items():
                            if not is_active and fps < threshold:
                                thresholds[threshold] = True
                                plt.scatter([spawned], [fps], color="red") # plotting single point
                                plt.text(spawned, fps, f" < {threshold}")

                        data_x.append(spawned)
                        data_y.append(fps)

            plt.plot(data_x, data_y, label=name)

    plt.title("Benchmark Reports")
    plt.legend()
    plt.xlabel("Spawned")
    plt.ylabel("FPS")
    plt.grid(which="major", axis="y", linestyle="dashed")
    plt.show()


if __name__ == "__main__":
    main()
