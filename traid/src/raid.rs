use crate::config::RaidConfig;
use std::io::{Read, Seek, SeekFrom, Write};

/// Decodes a hex string (e.g. `"deadbeef"`) into a byte vector.
///
/// Returns `Err(InvalidHex)` if the string contains non-hex characters or has an odd length.
fn parse_hex(s: &str) -> Result<Vec<u8>, crate::error::TraidError> {
    s.as_bytes()
        .chunks(2)
        .map(|chunk| {
            let pair = std::str::from_utf8(chunk)
                .map_err(|_| crate::error::TraidError::InvalidHex(s.to_string()))?;
            u8::from_str_radix(pair, 16)
                .map_err(|_| crate::error::TraidError::InvalidHex(s.to_string()))
        })
        .collect()
}

/// Computes the XOR parity of a set of equal-length byte blocks.
///
/// The result is the element-wise XOR of all blocks and is used both to write
/// the parity disk and to reconstruct a missing block during degraded reads.
fn xor_parity(blocks: &[&[u8]]) -> Vec<u8> {
    let mut parity = blocks[0].to_vec();
    for block in &blocks[1..] {
        for (p, b) in parity.iter_mut().zip(block.iter()) {
            *p ^= b;
        }
    }
    parity
}

/// Writes hex-encoded data to the RAID array starting at offset 0.
///
/// Data is split into stripes; each stripe is distributed across data disks and a
/// corresponding XOR parity block is written to the parity disk.
/// Returns `Err` if a disk is currently marked as failed.
pub fn cmd_write(data: String) -> Result<(), crate::error::TraidError> {
    let bytes = parse_hex(&data)?;
    // println!("Bytes {:?}", bytes);
    let config = RaidConfig::load()?;
    // println!("Config {:?}", config);

    if let Some(d) = config.failed_disk {
        return Err(crate::error::TraidError::DiskFailed(d));
    }

    let data_disks = (config.disks - 1) as usize;
    let stripe_size = data_disks * config.block_size as usize;

    // println!("data_disks {:?}", data_disks);
    // println!("stripe_size {:?}", stripe_size);

    for (stripe_idx, stripe) in bytes.chunks(stripe_size).enumerate() {
        // println!("stripe_idx {:?}, stripe {:?}", stripe_idx, stripe);
        let blocks: Vec<&[u8]> = stripe.chunks(config.block_size as usize).collect();
        // println!("blocks {:?}", blocks);
        let offset = (stripe_idx * config.block_size as usize) as u64;
        // println!("offset {:?}", offset);

        for (disk_idx, block) in blocks.iter().enumerate() {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .open(format!("disk_{}.bin", disk_idx))?;
            file.seek(SeekFrom::Start(offset))?;
            file.write_all(block)?;
        }

        let parity = xor_parity(&blocks);
        // println!("parity {:?}", parity);
        let mut pfile = std::fs::OpenOptions::new()
            .write(true)
            .open(format!("disk_{}.bin", config.disks - 1))?;
        pfile.seek(SeekFrom::Start(offset))?;
        pfile.write_all(&parity)?;
    }
    Ok(())
}

/// Reads `len` bytes from the array starting at `offset` and prints them as hex to stdout.
pub fn cmd_read(offset: u64, len: u64) -> Result<(), crate::error::TraidError> {
    let bytes = read_bytes(offset, len)?;
    for b in bytes {
        print!("{:02x}", b);
    }
    Ok(())
}

/// Reads `len` bytes from the logical array starting at byte `offset`.
///
/// In normal mode, blocks are read directly from the data disks.
/// In degraded mode (one failed disk), the missing block is reconstructed
/// on the fly using XOR parity of the remaining disks.
pub fn read_bytes(offset: u64, len: u64) -> Result<Vec<u8>, crate::error::TraidError> {
    let config = RaidConfig::load()?;
    let data_disks = (config.disks - 1) as u64;
    let stripe_size = config.block_size * data_disks;
    let stripe_start = offset / stripe_size;
    let stripe_end = (offset + len - 1) / stripe_size;
    let mut result: Vec<u8> = Vec::new();

    for stripe_idx in stripe_start..=stripe_end {
        if config.failed_disk.is_none() {
            // Read blocks from disks without parity 
            for disk_idx in 0..data_disks {
                let mut file = std::fs::OpenOptions::new()
                    .read(true)
                    .open(format!("disk_{}.bin", disk_idx))?;
                file.seek(SeekFrom::Start(stripe_idx * config.block_size))?;
                let mut block = vec![0u8; config.block_size as usize];
                file.read_exact(&mut block)?;
                result.append(&mut block);
            }
        } else {
            // disk is in degraded state, based on parity calculate missing part
            let mut blocks: Vec<Vec<u8>> = Vec::new();
            for disk_idx in 0..config.disks {
                if config.failed_disk != Some(disk_idx) {
                    let mut file = std::fs::OpenOptions::new()
                        .read(true)
                        .open(format!("disk_{}.bin", disk_idx))?;
                    file.seek(SeekFrom::Start(stripe_idx * config.block_size))?;
                    let mut block = vec![0u8; config.block_size as usize];
                    file.read_exact(&mut block)?;
                    blocks.push(block);
                }
            }
            let block_refs: Vec<&[u8]> = blocks.iter().map(|b| b.as_slice()).collect();
            let recovered = xor_parity(&block_refs);

            for disk_idx in 0..data_disks {
                if config.failed_disk != Some(disk_idx as u8) {
                    let mut file = std::fs::OpenOptions::new()
                        .read(true)
                        .open(format!("disk_{}.bin", disk_idx))?;
                    file.seek(SeekFrom::Start(stripe_idx * config.block_size))?;
                    let mut block = vec![0u8; config.block_size as usize];
                    file.read_exact(&mut block)?;
                    result.append(&mut block);
                } else {
                    result.extend_from_slice(&recovered);
                }
            }
        }
    }

    let start = (offset - stripe_start * stripe_size) as usize;
    let end = start + len as usize;
    Ok(result[start..end].to_vec())
}

/// Marks the given disk index as failed in the config, putting the array into degraded mode.
pub fn cmd_fail(disk: u8) -> Result<(), crate::error::TraidError> {
    let mut config = RaidConfig::load()?;
    config.failed_disk = Some(disk);
    config.save()?;
    Ok(())
}

/// Reconstructs a failed disk by XOR-ing all remaining disks stripe by stripe,
/// then clears the failed-disk flag in the config.
pub fn cmd_rebuild(disk: u8) -> Result<(), crate::error::TraidError> {
    let mut config = RaidConfig::load()?;
    let stripes = config.disk_size / config.block_size;

    for stripe_idx in 0..stripes {
        let mut blocks: Vec<Vec<u8>> = Vec::new();
        for disk_idx in 0..config.disks {
            if disk_idx != disk {
                let mut file = std::fs::OpenOptions::new()
                    .read(true)
                    .open(format!("disk_{}.bin", disk_idx))?;
                file.seek(SeekFrom::Start(stripe_idx * config.block_size))?;
                let mut block = vec![0u8; config.block_size as usize];
                file.read_exact(&mut block)?;
                blocks.push(block);
            }
        }
        let block_refs: Vec<&[u8]> = blocks.iter().map(|b| b.as_slice()).collect();
        let recovered = xor_parity(&block_refs);
        let offset = stripe_idx * config.block_size;

        let mut pfile = std::fs::OpenOptions::new()
            .write(true)
            .open(format!("disk_{}.bin", disk))?;
        pfile.seek(SeekFrom::Start(offset))?;
        pfile.write_all(&recovered)?;
    }
    config.failed_disk = None;
    config.save()?;
    Ok(())
}

/// Prints the current array configuration and health status (OK or Degraded).
pub fn cmd_status() -> Result<(), crate::error::TraidError> {
    let config = RaidConfig::load()?;
    println!("Disks: {}", config.disks);
    println!("Block size: {}", config.block_size);
    println!("Disk size: {}", config.disk_size);

    match config.failed_disk {
        None => println!("Status: OK"),
        Some(d) => println!("Status: Degraded (disk {} failed)", d),
    }

    Ok(())
}

/// Initialises a new RAID array: writes `raid.json` config and creates zero-filled
/// `disk_N.bin` files for each of the `disks` virtual drives.
pub fn cmd_init(
    disks: u8,
    block_size: u64,
    disk_size: u64,
) -> Result<(), crate::error::TraidError> {
    let raid_config = RaidConfig {
        disks,
        block_size,
        disk_size,
        failed_disk: None,
    };
    raid_config.save()?;
    for i in 0..disks {
        let file = std::fs::File::create(format!("disk_{}.bin", i))?;
        file.set_len(disk_size)?;
    }
    Ok(())
}
