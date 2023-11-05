from datetime import datetime
from pathlib import Path
import shutil
import subprocess


EXECUTABLE = "shooter.exe"
CONFIG = "config.toml"
ASSETS = "assets"


def main():
    print("Preparing...")
    name = datetime.utcnow().strftime("shooter_rs-%Y%m%d-win64")
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
    shutil.copyfile(root.joinpath(CONFIG), output.joinpath(CONFIG))
    shutil.copytree(root.joinpath(ASSETS), output.joinpath(ASSETS))

    print("Zipping...")
    shutil.make_archive(output, "zip", output)

    print("Done!")


if __name__ == "__main__":
    main()
