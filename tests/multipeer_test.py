import os
import pathlib
import shutil
import time

N_PEERS = 2


def test_multipeer_deployment(tmp_path: pathlib.Path):
    # Ensure current path is project root
    directory_path = os.getcwd()
    folder_name = os.path.basename(directory_path)
    assert folder_name == "qdrant"

    # Make peer folders
    execs = []
    for i in range(N_PEERS):
        peer_dir = tmp_path / f"peer{i}"
        peer_dir.mkdir()
        execs.append(shutil.copy("./target/debug/qdrant", peer_dir))
    print(execs)

    # Start peers (with env variables?)

    # Start bootstrap

    time.sleep(10)
    # Start others

    # Wait
    time.sleep(10)

    # Create collection

    # Check that it exists on all peers

    # TODO: Provide API to get raft state data on each peer
