import datetime
import matplotlib.pyplot as plt
import os
import subprocess
import time


DIRECTORY = "./bench"
PASSES = 0  # 0 - to view only
LABEL = ""


def main():
    if PASSES > 0:
        source = os.path.join(DIRECTORY, "temp")
        clear_report_files(source)

        for _ in range(PASSES):
            print("Sleeping...")
            time.sleep(2)
            # TODO: pass bench flag
            # TODO: pass output dir
            subprocess.run(["cargo", "run", "--release"])

        output_name = datetime.datetime.now(datetime.UTC).strftime("%Y-%m-%d %H-%M-%S")

        if LABEL:
            output_name = "{} {}".format(output_name, LABEL)

        output = os.path.join(DIRECTORY, output_name)
        merge_reports(source, output)

    show_report(DIRECTORY)


def merge_reports(source_directory, output_file):
    data = {}

    for name, path in iter_report_files(source_directory):
        for spawned, fps in iter_report_lines(path):
            if spawned not in data:
                data[spawned] = []

            data[spawned].append(fps)

    if not data:
        return

    with open(output_file, "w") as file:
        for spawned in sorted(data.keys()):
            fps_all = data[spawned]
            fps_avg = sum(fps_all) / len(fps_all)
            file.write("{},{}\n".format(spawned, int(fps_avg)))


def show_report(directory):
    for name, path in iter_report_files(directory):
        thresholds = {
            240: False,
            120: False,
            60: False,
            30: False,
        }

        data_x = []
        data_y = []

        for spawned, fps in iter_report_lines(path):
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


def clear_report_files(directory):
    for _, path in iter_report_files(directory):
        os.remove(path)


def iter_report_files(directory):
    if not os.path.isdir(directory):
        return

    for name in os.listdir(directory):
        path = os.path.join(directory, name)

        if os.path.isfile(path):
            yield name, path


def iter_report_lines(path):
    with open(path) as file:
        for line in file:
            line = line.strip()  # remove new line character

            if line:
                spawned, fps = line.split(",")
                yield int(spawned), int(fps)


if __name__ == "__main__":
    main()
