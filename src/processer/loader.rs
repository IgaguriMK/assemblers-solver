#[cfg(test)]
mod test;

use std::fs::File;

use failure::Error;
use serde::Deserialize;
use serde_yaml::from_reader;

use super::Processer;

const PROC_MOD_PROC: f64 = 0.1;
const PROC_MOD_SPEED: f64 = -0.15;
const SPEED_MOD_SPPEED: f64 = 0.5;
const BEACON_SPEED: f64 = SPEED_MOD_SPPEED;

pub fn load() -> Result<Vec<Processer>, Error> {
    let f = File::open("./data/processers.yaml")?;

    let proc_types: Vec<ProcType> = from_reader(f)?;

    let mut res = Vec::new();

    for t in proc_types {
        let mods_list = gen_ps_tuples(t.max_modules);

        for c in &t.configulations {
            for m in &mods_list {
                if m.0 + m.1 < t.max_modules && c.beacon > 0 {
                    continue;
                }

                let p = build_proc(&t.name, t.base_speed, m, c);
                res.push(p);
            }
        }
    }

    Ok(res)
}

fn build_proc(
    base_name: &str,
    base_speed: f64,
    mods: &(usize, usize),
    conf: &Configulation,
) -> Processer {
    let &(proc_cnt, speed_cnt) = mods;

    let speed_mult = 1.0
        + (proc_cnt as f64) * PROC_MOD_SPEED
        + (speed_cnt as f64) * SPEED_MOD_SPPEED
        + (conf.beacon as f64) * BEACON_SPEED;
    let speed = base_speed * speed_mult;

    let productivity = 1.0 + (proc_cnt as f64) * PROC_MOD_PROC;

    let mut name = base_name.to_string();

    if proc_cnt > 0 {
        name += &format!("-p{}", proc_cnt);
    }

    if speed_cnt > 0 {
        name += &format!("-s{}", speed_cnt);
    }

    if conf.beacon > 0 {
        name += &format!("-b{}", conf.beacon);
    }

    Processer {
        name,
        proc_type: base_name.to_string(),
        productivity,
        speed,
        io: conf.io,
        speed_module: speed_cnt,
        productivity_module: proc_cnt,
        beacon: conf.beacon,
    }
}

#[derive(Debug, Deserialize)]
struct ProcType {
    name: String,
    base_speed: f64,
    max_modules: usize,
    configulations: Vec<Configulation>,
}

#[derive(Debug, Deserialize)]
struct Configulation {
    beacon: usize,
    io: usize,
}

fn gen_ps_tuples(max: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    // Allow always (0, 0)
    res.push((0, 0));

    if max > 0 {
        // all prod
        res.push((max, 0));

        // 'PPPS' configulation
        if max >= 2 {
            res.push((max - 1, 1));
        }

        // speed modules only
        for i in 1..=max {
            res.push((0, i));
        }
    }

    res
}
