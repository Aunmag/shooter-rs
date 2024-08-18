from pathlib import Path
import datetime
import shutil
import subprocess


EXECUTABLE = "shooter.exe"
SETTINGS = "settings.toml"
ASSETS = "assets"
GIT_TAG = False


def main():
    print("Preparing...")
    date = datetime.datetime.now(datetime.UTC).strftime("%Y%m%d")
    name = "A Zombie Shooter Game (build {})".format(date)
    name_zip = "shooter_rs-{}-win64".format(date)
    root = Path(__file__).parent.resolve()
    target = root.joinpath("target")
    output = target.joinpath(name)

    print("Building...")
    subprocess.run(
        ["cargo", "build", "--release", "--manifest-path", str(root.joinpath("Cargo.toml"))],
        check=True,
    )

    print("Copying files...")
    if output.exists():
        shutil.rmtree(output)
    output.mkdir()
    shutil.copyfile(target.joinpath("release").joinpath(EXECUTABLE), output.joinpath(EXECUTABLE))
    shutil.copyfile(root.joinpath(SETTINGS), output.joinpath(SETTINGS))
    shutil.copytree(root.joinpath(ASSETS), output.joinpath(ASSETS))

    print("Zipping...")
    shutil.make_archive(target.joinpath(name_zip), "zip", target, name)

    if GIT_TAG:
        print("Tagging...")
        subprocess.run(["git", "tag", date], check=True)
        subprocess.run(["git", "push", "origin", date], check=True)

    print("Done!")


if __name__ == "__main__":
    main()
