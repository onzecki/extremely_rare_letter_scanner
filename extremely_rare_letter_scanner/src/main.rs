use std::fs;
use std::io;
use colored::Colorize;

fn main() {
    let rare_combinations = fs::read_to_string("rare.txt").unwrap_or("bk fq jc jt mj qh qx vj zh bq fv jd jv mq qj vk xb zj bx fx jf jw mx vm xg zn cb fz jg jx mz ql sx vn xj zq cf gq jh jy pq qm sz vp xk zr cg gv jk jz pv qn tq vq xv zs cj gx jl kq px qo tx vt xz zx cp hk jm kv qb qp vb vw yq cv hv jn kx qc qr vc vx yv cw hx jp kz qd qs vd vz yz cx hz lq qe qt vf zb dx iy jr lx qf qv vg wv zc fk jb js mg qw vh wx zg".to_string());
    let ultra_rare_combinations =
        fs::read_to_string("ultra_rare.txt").unwrap_or("jq qg qk qy qz wq wz".to_string());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    for i in 0..input.len() - 2 {
        let str = &input[i..i + 2]
            .trim()
            .trim_matches(char::is_numeric)
            .to_lowercase();
        if !(str.len() == 2) {
            continue;
        }
        if ultra_rare_combinations.contains(str) {
            println!("▶ {} is {}", str.italic(), "ultra rare!".purple().bold());
            continue;
        }
        if rare_combinations.contains(str) {
            println!("▶ {} is {}", str.italic(), "rare!".bright_blue().bold());
            continue;
        }
    }
}
