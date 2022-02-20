use bevy::prelude::*;
use std::collections::HashMap;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub struct PassLayer{
    target: usize,
    from: usize,
    passes: u32,
    seeds: u32,
}

impl PassLayer {
    pub fn from_arr(arr: &[(usize,usize,u32,u32)])->Vec<PassLayer>{
        let mut v_p_l = Vec::new();

        for &(target,from,passes,seeds) in arr {
            v_p_l.push(PassLayer{target,from,passes,seeds});
        }

        v_p_l
    }
}

pub fn world_from_image(
    texture: &Image,
    palette: &Image
) -> (Vec<usize>,u32,u32)
{
    let size = &texture.texture_descriptor.size;
    let data = &texture.data;
    let mut out = Vec::new();

    let p_data = palette.data.chunks(4);

    let mut hash_map = HashMap::new();

    for (i,p) in p_data.enumerate() {
        hash_map.insert(p,i);
    }

    for p in data.chunks(4) {
        let i = hash_map.get(p).unwrap();
        out.push(*i);
    }

    (out,size.width,size.height)
}

pub fn world_from_seed(seed: u64, width: usize, height: usize, layers: Vec<PassLayer>) -> (Vec<usize>,u32,u32){
    let mut r = StdRng::seed_from_u64(seed);

    let mut out = vec![1;width * height];

    //out[width/2 + height/2 * width] = 0;

    //println!("{:?}",out);

    //for pass in 0..passes {
        //out = apply_single_pass(out,width,target,&mut r);
    //}
    for p_l in layers {
        out = apply_all_pass_on_target(out,p_l.passes,width,&mut r,p_l.target, p_l.from,p_l.seeds);
    }

    (out,width as u32,height as u32)
}

fn apply_all_pass_on_target(
    map : Vec<usize>,
    passes: u32,
    width: usize,
    rand : &mut StdRng,
    target : usize,
    from : usize,
    seeds: u32,
) -> Vec<usize>{
    let mut new_map = map.clone();

    for _ in 0..seeds {
        let i;

        let mut from_map: Vec<(usize,&usize)> = new_map.iter().enumerate().filter(|(_,&x)| x == from).collect();

        let r_i: usize = rand.gen_range(0..from_map.len());
        i = from_map[r_i].0;

        new_map[i] = target;
    }

    for pass in 0..passes {
        new_map = apply_single_pass(new_map,width,target, rand,from);
    }

    new_map
}

fn apply_single_pass(
    map : Vec<usize>,
    width: usize,
    target: usize,
    rand: &mut StdRng,
    from: usize,
) -> Vec<usize>{
    let mut new_map = map.clone();
    for (i,&p) in map.iter().enumerate() {
        if p == from {

            let pre;
            if i > 0{pre = map.get(i - 1);}
            else{ pre = None;}

            let under;
            if i >= width {under = map.get(i - width);}
            else {under = None;}

            let neighbours = vec![map.get(i + 1), pre, map.get(i + width), under];

            for n in neighbours {
                match n {
                    Some(&n_p) => {
                        if n_p == target {
                            let r: u32 = rand.gen_range(0..100);
                            if r < 70 {
                                new_map[i] = target;
                            }
                        }
                    }
                    None => {}
                }
            }
        }
    }
    new_map
}