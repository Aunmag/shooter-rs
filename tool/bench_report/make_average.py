import matplotlib.pyplot as plt
import os

REPORTS_DIRECTORY = "reports"


def main():
    data = {}

    for name in os.listdir(REPORTS_DIRECTORY):
        path = os.path.join(REPORTS_DIRECTORY, name)

        if os.path.isfile(path):
            with open(path) as file:
                for line in file:
                    line = line.strip()  # remove new line character

                    if line:
                        spawned, fps = line.split(",")
                        spawned = int(spawned)
                        fps = int(fps)

                        if spawned not in data:
                            data[spawned] = []

                        data[spawned].append(fps)

    with open(os.path.join(REPORTS_DIRECTORY, "avg"), "w") as file:
        for spawned in sorted(data.keys()):
            fps_all = data[spawned]
            fps_avg = sum(fps_all) / len(fps_all)
            file.write("{},{}\n".format(spawned, int(fps_avg)))


if __name__ == "__main__":
    main()
