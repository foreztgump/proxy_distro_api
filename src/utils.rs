use rand::Rng;
use std::fs::File;
use std::{io, fs};
use std::path::Path;
use chrono::{DateTime, Utc};
use std::string::ParseError;
use std::io::{BufRead, BufReader, Write};
use rand::{seq::SliceRandom, thread_rng};
use crate::constants::{PROXY_OUT, PROXY_USED};


pub fn get_proxy(country_code: String) -> Result<String, ParseError> {
    //println!("Start Getting Proxy");
    let mut country = String::new();
    let time_now = Utc::now();
    let mut proxy_send = String::new();
    let mut proxy_to_send_raw = String::new();
    let mut vec_country = Vec::new();

    //println!("Read Good Proxy");
    let mut vec_list = Vec::new();
    match lines_from_file(PROXY_OUT) {
        Ok(k) => { vec_list = k },
        Err(e) => { println!("{}", e) }
    }

    //println!("Read Used Proxy");
    let mut vec_used_proxy =  Vec::new();
    match lines_from_file(PROXY_USED) {
        Ok(k) => { vec_used_proxy = k },
        Err(e) => { println!("{}", e) }
    }

    //Filter request country to vec
    //println!("Filter Country From Request");
    for x in vec_list.iter() {
        if x.contains(&country) {
            vec_country.push(x)
        }
    }

    //Update Used Proxy Vec
    let mut vec_to_write = Vec::new();
    let mut vec_to_check = Vec::new();
    if !vec_used_proxy.is_empty() {
        //println!("Read Time in Used Proxy");

        for x in vec_used_proxy.iter() {
            let mut time = String::new();
            let mut line_to_check = String::new();
            if x.len() > 1 {
                let vec_split: Vec<String> = x.split(":").map(|s|s.to_string()).collect();
                time = format!("{}:{}:{}", vec_split[3], vec_split[4], vec_split[5]);
                line_to_check = format!("{}:{}:{}", vec_split[0], vec_split[1], vec_split[2]);
            }
            match time.parse::<DateTime<Utc>>() {
                Ok(k) => {
                    let parse_dt = k;
                    //println!("{}", parse_dt);
                    //println!("Time passed {}", time_now.signed_duration_since(parse_dt).num_minutes());
                    if time_now.signed_duration_since(parse_dt).num_minutes() <= 5 {
                        //println!("Keep Old Proxy");
                        vec_to_write.push(x.clone());
                        vec_to_check.push(line_to_check);
                        //vec_used_proxy.remove(count);
                    }
                },
                Err(e) => { println!("Failed Parsed Time : {}", e) }
            }
        }
    }

    //parse query
    if country_code.contains("=") {
        //Browser Request
        let vec_cc: Vec<String> = country_code.split("=").map(|x| x.to_string()).collect();
        let proxy_kind = vec_cc[1].clone();
        country = vec_cc[0].clone();

        if proxy_kind == "res" {
            //Residential Proxy request
            //Filter proxy to send by country when
            //there are proxies for the country
            let mut count= 0;
            //println!("Get Proxy to Send");
            while !vec_country.is_empty() {
                let chose_rand = vec_country.choose(&mut thread_rng()).unwrap().to_string();
                //println!("{}", chose_rand);
                //Check if random proxy is already in used
                if !vec_to_check.contains(&chose_rand) {
                    //Not used
                    proxy_to_send_raw = chose_rand;
                    break;
                } else if count >= vec_to_check.len() {
                    proxy_to_send_raw = random_geek_proxy(&country);
                    break;
                }
                count += 1;
                //println!("Getting new proxy");
            }

            //Don't have proxy for the country to send
            if vec_country.is_empty() {
                //println!("Get Geek Proxy to Send");
                proxy_to_send_raw = random_geek_proxy(&country);
            }

            //prepare proxy_to_send to ip:port
            //println!("Preparing Proxy to Send");
            if !proxy_to_send_raw.is_empty() {
                let vec_split: Vec<String> = proxy_to_send_raw.split(":").map(|s|s.to_string()).collect();
                proxy_send = format!("{}:{}", vec_split[0], vec_split[1]);
            }
        } else if proxy_kind == "geek" {
            //Geek proxy request
            proxy_send = random_geek_proxy(&country);
        }
    } else {
        //Bot Request
        country = country_code;

        //Filter proxy to send by country when
        //there are proxies for the country
        let mut count= 0;
        //println!("Get Proxy to Send");
        while !vec_country.is_empty() {
            let chose_rand = vec_country.choose(&mut thread_rng()).unwrap().to_string();
            //println!("{}", chose_rand);
            //Check if random proxy is already used
            if !vec_to_check.contains(&chose_rand) {
                //Not used
                let new_line = format!("{}:{}", chose_rand, Utc::now());
                proxy_to_send_raw = chose_rand;
                vec_to_write.push(new_line);
                break;
            } else if count >= vec_to_check.len() {
                proxy_to_send_raw = random_geek_proxy(&country);
                break;
            }
            count += 1;
            //println!("Getting new proxy");
        }

        //Don't have proxy for the country to send
        if vec_country.is_empty() {
            //println!("Get Geek Proxy to Send");
            proxy_to_send_raw = random_geek_proxy(&country);
        }

        //prepare proxy_to_send to ip:port
        //println!("Preparing Proxy to Send");
        if !proxy_to_send_raw.is_empty() {
            let vec_split: Vec<String> = proxy_to_send_raw.split(":").map(|s|s.to_string()).collect();
            proxy_send = format!("{}:{}", vec_split[0], vec_split[1]);
        }


        //println!("vec_used_proxy : {:?}", vec_to_write);
        //Update Used Proxy file
        let _file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(PROXY_USED)
            .unwrap();

        for x in vec_to_write.iter() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(PROXY_USED)
                .unwrap();
            writeln!(file, "{}", x).unwrap();
        }
    }



    Ok(proxy_send)
}

pub fn random_geek_proxy(country_code: &str) -> String {
    let mut proxy_to_send_raw = String::new();
    let country = country_code;
    if country == "US" {
        let mut rng = rand::thread_rng();
        let proxy = format!("135.181.60.66:{}", rng.gen_range(7000..7060));
        proxy_to_send_raw = proxy;
    } else if country == "DE" {
        let mut rng = rand::thread_rng();
        let proxy = format!("135.181.60.66:{}", rng.gen_range(9000..9019));
        proxy_to_send_raw = proxy;
    } else if country == "IT" {
        let mut rng = rand::thread_rng();
        let proxy = format!("135.181.60.66:{}", rng.gen_range(9020..9024));
        proxy_to_send_raw = proxy;
    } else if country == "NL" {
        let mut rng = rand::thread_rng();
        let proxy = format!("135.181.60.66:{}", rng.gen_range(9025..9034));
        proxy_to_send_raw = proxy;
    } else if country == "UK" {
        let mut rng = rand::thread_rng();
        let proxy = format!("135.181.60.66:{}", rng.gen_range(9035..9039));
        proxy_to_send_raw = proxy;
    } else {
        //proxy_to_send_raw = String::from("none");
    }
    proxy_to_send_raw
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}