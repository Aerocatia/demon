use crate::crc32::CRC32;
use crate::init::{get_exe_type, ExeType};
use crate::multiplayer::map_list::{get_mp_map_data_by_index, header_from_cache};
use alloc::format;
use alloc::vec::Vec;
use c_mine::c_mine;
use minxp::fs::File;
use minxp::io::{Read, Seek, SeekFrom};
use minxp::path::PathBuf;
use tag_structs::{CacheFileTag, CacheFileTagDataHeader, CacheFileTagDataHeaderExternalModels, Scenario, ScenarioBSP};

#[c_mine]
pub unsafe extern "C" fn get_mp_map_crc32(map: usize) -> u32 {
    let Some(map_data) = get_mp_map_data_by_index(map) else {
        panic!("get_mp_map_crc32: tried to get CRC32 of MP map {map} but it was out-of-bounds")
    };

    if map_data.crc_verified {
        return map_data.crc32;
    }

    let map_name = map_data.name.expect_str();
    let crc32 = match get_map_crc32(map_name) {
        Ok(n) => n,
        Err(e) => panic!("get_mp_map_crc32: tried to get CRC32 of MP map {map_name} but got an error: {e}")
    };

    map_data.crc32 = crc32;
    map_data.crc_verified = true;

    crc32
}

pub fn get_map_crc32(map_name: &str) -> Result<u32, &'static str> {
    if get_exe_type() != ExeType::Cache {
        warn!("Cannot get CRC32 of {map_name}: not on a cache build (using u32::MAX)");
        return Ok(u32::MAX)
    };

    let mut crc = CRC32::new();
    let header = header_from_cache(map_name)?;

    let mut tag_data: Vec<u8> = Vec::new();
    let expected_tag_data_size = header.tag_data_size as usize;
    if expected_tag_data_size < size_of::<CacheFileTagDataHeader>() {
        return Err("invalid tag data size");
    }
    tag_data.try_reserve_exact(expected_tag_data_size).expect("Not enough RAM to check CRC32 (tag data)");
    tag_data.resize(expected_tag_data_size, 0);

    let path = PathBuf::from(format!("maps\\{map_name}.map"));
    let Ok(mut file) = File::open(path) else { return Err("cannot read map") };

    fn read_at(file: &mut File, at: usize, to: &mut [u8]) -> Result<(), &'static str> {
        (|| -> minxp::io::Result<()> {
            assert_eq!(at, file.seek(SeekFrom::Start(at as u64))? as usize);
            file.read_exact(to)?;
            Ok(())
        })().map_err(|_| "failed to read tag data")
    }

    read_at(&mut file, header.tag_data_offset as usize, tag_data.as_mut_slice())?;

    let base_memory_address = 0x40440000;

    fn translate_ptr<T: Sized>(what: &[u8], ptr: u32, count: usize) -> Option<&[T]> {
        let element_size = size_of::<T>();
        let needed_size = element_size.checked_mul(count)?;
        let ptr = ptr.checked_sub(0x40440000)? as usize;
        let ptr_end = ptr.checked_add(needed_size)?;
        let data = what.get(ptr..ptr_end)?;
        unsafe { Some(core::slice::from_raw_parts(data.as_ptr() as *const T, count)) }
    }

    let Some([tag_data_header_e]) = translate_ptr::<CacheFileTagDataHeaderExternalModels>(tag_data.as_slice(), base_memory_address, 1) else {
        return Err("Can't read tag data header")
    };
    let tag_data_header = tag_data_header_e.cache_file_tag_data_header;
    let tag_count = tag_data_header.tag_count as usize;
    let scenario_tag_index = (tag_data_header.scenario_tag.0 & 0xFFFF) as usize;
    let q = tag_data_header.tag_array_address;

    let Some(tag_array) = translate_ptr::<CacheFileTag>(tag_data.as_slice(), tag_data_header.tag_array_address.0, tag_count) else {
        return Err("Can't read tag entries")
    };
    let Some(scenario_tag) = tag_array.get(scenario_tag_index) else {
        return Err("Can't get scenario tag")
    };
    let Some([scenario_tag_data]) = translate_ptr::<Scenario>(tag_data.as_slice(), scenario_tag.data.0, 1) else {
        return Err("Can't read scenario tag");
    };
    let bsp_reflexive = scenario_tag_data.structure_bsps;
    let Some(bsps) = translate_ptr::<ScenarioBSP>(tag_data.as_slice(), bsp_reflexive.address.0, bsp_reflexive.count as usize) else {
        return Err("Can't read BSP reflexive")
    };

    let mut work_data: Vec<u8> = Vec::new();
    for i in bsps {
        work_data.clear();
        let bsp_size = i.bsp_size as usize;
        let Ok(()) = work_data.try_reserve_exact(bsp_size) else {
            return Err("Not enough RAM to check BSPs")
        };
        work_data.resize(bsp_size, 0);
        read_at(&mut file, i.bsp_start as usize, work_data.as_mut_slice())?;
        crc.update(work_data.as_slice());
    }
    work_data.clear();

    let model_size = tag_data_header_e.model_data_size as usize;
    let model_offset = tag_data_header_e.model_data_file_offset as usize;
    work_data.resize(model_size, 0);
    read_at(&mut file, model_offset, work_data.as_mut_slice())?;

    crc.update(work_data.as_slice());
    crc.update(tag_data.as_slice());

    let crc_header = header.crc32;
    let crc = crc.crc();
    if header.crc32 != crc {
        warn!("get_mp_map_crc32: map {map_name} has a mismatched CRC32 (likely modified)");
        warn!("   HEADER: 0x{crc_header:08X}; CALCULATED: 0x{crc:08X}");
        warn!("   This will be a hard error on a future version!");
    }

    Ok(crc)
}
