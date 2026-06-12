"""Fake disk data so the app works without root / real hardware."""

DISKS = [
    {
        "name": "sda",
        "model": "Samsung 870 EVO",
        "size_gb": 500,
        "temp_c": 34,
        "health": 98,
        "smart_status": "PASSED",
        "reallocated_sectors": 0,
        "io_read_mb": 120.4,
        "io_write_mb": 45.2,
    },
    {
        "name": "sdb",
        "model": "WD Blue 1TB",
        "size_gb": 1000,
        "temp_c": 41,
        "health": 72,
        "smart_status": "PASSED",
        "reallocated_sectors": 3,
        "io_read_mb": 88.1,
        "io_write_mb": 210.0,
    },
    {
        "name": "sdc",
        "model": "Seagate Barracuda",
        "size_gb": 2000,
        "temp_c": 55,
        "health": 31,
        "smart_status": "FAILED",
        "reallocated_sectors": 47,
        "io_read_mb": 12.0,
        "io_write_mb": 3.3,
    },
]
