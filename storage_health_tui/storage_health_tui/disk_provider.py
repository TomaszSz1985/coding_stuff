import subprocess
import json


def get_disks() -> list[dict]:
    disks = []
    result = subprocess.run(["lsblk", "-J", "-o", "NAME,SIZE,MODEL,TYPE"], capture_output=True)
    data = json.loads(result.stdout)
    for device in data["blockdevices"]:
        if device["type"] == "disk":
            disk = {
                "name": device["name"],
                "model": device["model"] or "Unknown",
                "size_gb": device["size"],
                "temp_c": 0,  # placeholder — dodamy przez smartctl
                "health": 100,  # placeholder
                "smart_status": "UNKNOWN",
                "reallocated_sectors": 0,
                "io_read_mb": 0.0,
                "io_write_mb": 0.0,
            }
            smart = get_smart(device["name"])
            disk.update(smart)

            disks.append(disk)
    return disks


def get_smart(name: str) -> dict:
    result = subprocess.run(["smartctl", "-j", "-a", f"/dev/{name}"], capture_output=True)
    data = json.loads(result.stdout)

    result = {
        "temp_c": data["temperature"]["current"],
        "health": 100 - data["nvme_smart_health_information_log"]["percentage_used"],  # placeholder
        "smart_status": "PASSED" if data["smart_status"]["passed"] else "FAILED",
        "reallocated_sectors": data["nvme_smart_health_information_log"]["media_errors"],
    }

    return result


if __name__ == "__main__":
    result = get_disks()
    print(result)
