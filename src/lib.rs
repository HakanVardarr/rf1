use serde_json::Value;
use std::error::Error;

use std::env;

fn get_value(url: &str) -> Result<Value, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    Ok(serde_json::from_slice(resp.as_bytes())?)
}

fn print_constructor_standings() -> Result<(), Box<dyn Error>> {
    let v: Value = get_value("https://ergast.com/api/f1/current/constructorStandings.json")?;
    let list = &v["MRData"]["StandingsTable"]["StandingsLists"][0]["ConstructorStandings"];
    let count = v["MRData"]["total"]
        .as_str()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    for i in 0..count {
        let points = list[i]["points"].as_str().unwrap();
        let name = list[i]["Constructor"]["name"].as_str().unwrap();
        if i == 9 {
            println!("{}. {name} = {points}", i + 1,);
        } else {
            println!("{}.  {name} = {points}", i + 1);
        }
    }

    Ok(())
}

fn print_driver_standings() -> Result<(), Box<dyn Error>> {
    let v: Value = get_value("https://ergast.com/api/f1/current/driverStandings.json")?;
    let list = &v["MRData"]["StandingsTable"]["StandingsLists"][0]["DriverStandings"];
    let count = v["MRData"]["total"]
        .as_str()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    for i in 0..count {
        let points = &list[i]["points"].as_str().unwrap();
        let name = &list[i]["Driver"]["givenName"].as_str().unwrap();
        let surname = &list[i]["Driver"]["familyName"].as_str().unwrap();
        if i >= 9 {
            println!("{}. {name} {surname} = {points}", i + 1,);
        } else {
            println!("{}.  {name} {surname} = {points}", i + 1);
        }
    }

    Ok(())
}

fn print_schedule() -> Result<(), Box<dyn Error>> {
    let v: Value = get_value("https://ergast.com/api/f1/current.json")?;
    let list = &v["MRData"]["RaceTable"]["Races"];
    let count = v["MRData"]["total"]
        .as_str()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    for i in 0..count {
        let name = list[i]["raceName"].as_str().unwrap();
        let date = list[i]["date"].as_str().unwrap();
        let time = list[i]["time"].as_str().unwrap();
        let time = &time[0..time.len() - 1];

        if i >= 9 {
            println!("{}. {name} = {date} / {time} GMT", i + 1);
        } else {
            println!("{}.  {name} = {date} / {time} GMT", i + 1);
        }
    }

    Ok(())
}

fn print_last_race() -> Result<(), Box<dyn Error>> {
    let v: Value = get_value("http://ergast.com/api/f1/current/last/results.json")?;
    let table = &v["MRData"]["RaceTable"]["Races"][0]["Results"];
    let count = v["MRData"]["total"]
        .as_str()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    println!("-----PODIUM-------");
    for i in 0..count {
        let name = table[i]["Driver"]["givenName"].as_str().unwrap();
        let surname = table[i]["Driver"]["familyName"].as_str().unwrap();
        if i >= 9 {
            println!("{}. {name} {surname}", i + 1);
        } else {
            println!("{}.  {name} {surname}", i + 1);
            if i == 2 {
                println!("------------------");
            }
        }
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if args[1].to_uppercase() == String::from("CONSTRUCTOR") {
            print_constructor_standings()?;
        } else if args[1].to_uppercase() == String::from("DRIVER") {
            print_driver_standings()?;
        } else if args[1].to_uppercase() == String::from("SCHEDULE") {
            print_schedule()?;
        } else if args[1].to_uppercase() == String::from("LAST_RACE") {
            print_last_race()?;
        } else {
            println!("ERROR: There is no command such as: {}", args[1]);
        }
    } else {
        println!("USAGE:");
        println!("      rf1 constructor -> prints out constructor standings");
        println!("      rf1 driver     -> prints out driver standings");
        println!("      rf1 schedule    -> prints out f1 schedule");
        println!("      rf1 last_race   -> prints out results for last_race");
    }

    Ok(())
}
