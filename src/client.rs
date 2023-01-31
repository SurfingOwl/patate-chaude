use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::io::{self,BufRead,prelude::*,BufReader,Write};
use std::net::TcpStream;
use std::{thread,str};
use serde_json::json;
use byteorder::{ByteOrder, BigEndian};
use crate::shared;
use crate::shared::*;
#[derive(Serialize, Deserialize)]
enum Message {
    Subscribe{name:String}
}

fn sendData(data: impl AsRef<[u8]>, mut stream: impl io::Write)
    -> io::Result<()>
{
    let data = data.as_ref();
    let mut buf = [0u8; 4];
    BigEndian::write_u32(&mut buf, data.len() as u32);
    stream.write_all(&buf);
    stream.write_all(data);
    Ok(())
}

fn exchange_with_server(mut stream: TcpStream) {
    sendData(serde_json::to_string("Hello").unwrap(),&stream);
    let mut welcomesize = &mut [0u8; 4];
    match stream.read(welcomesize) {
        Ok(received) => {
            if received < 1 {
                println!("Perte de la connexion avec le serveur");
                return;
            }
        }
        Err(_) => {
            println!("Perte de la connexion avec le serveur");
            return;
        }
    }
    println!("Read message size : {:?}", welcomesize[3]);
    let mut welcomemessage = &mut [0u8;25];
    
    match stream.read(welcomemessage) {
        Ok(received) => {
            if received < 1 {
                println!("Perte de la connexion avec le serveur");
                return;
            }
        }
        Err(_) => {
            println!("Perte de la connexion avec le serveur");
            return;
        }
    }
    let welcomemessage_string = str::from_utf8(welcomemessage).unwrap();
    println!("Welcome: {}", welcomemessage_string);

    let subscribe = Message::Subscribe{name:"petit_test".to_string()};
    //println!("{}",serde_json::to_string(&subscribe).unwrap());
    sendData(serde_json::to_string(&subscribe).unwrap(),&stream);

    let mut subscriberesultsize = &mut [0u8;4];
    
    match stream.read(subscriberesultsize) {
        Ok(received) => {
            if received < 1 {
                println!("Perte de la connexion avec le serveur");
                return;
            }
        }
        Err(_) => {
            println!("Perte de la connexion avec le serveur");
            return;
        }
    }
    println!("Read message size : {:?}", subscriberesultsize[3]);

    let mut subscriberesultmessage = vec! [0u8;usize::from(subscriberesultsize[3])];
    
    match stream.read(&mut subscriberesultmessage) {
        Ok(received) => {
            if received < 1 {
                println!("Perte de la connexion avec le serveur");
                return;
            }
        }
        Err(_) => {
            println!("Perte de la connexion avec le serveur");
            return;
        }
    }
    let subscriberesultmessage_string = str::from_utf8(&subscriberesultmessage).unwrap();
    println!("SubscribeResult: {}", subscriberesultmessage_string);

    loop {
        let mut publicleaderboardmessagesize = &mut [0u8;4];
        
        match stream.read(publicleaderboardmessagesize) {
            Ok(received) => {
                if received < 1 {
                    println!("Perte de la connexion avec le serveur");
                    return;
                }
            }
            Err(_) => {
                println!("Perte de la connexion avec le serveur");
                return;
            }
        }
        println!("Read public leader board message size : {:?}", publicleaderboardmessagesize[3]);

        let mut publicleaderboardmessage = vec![0u8;usize::from(publicleaderboardmessagesize[3])];
        
        match stream.read(&mut publicleaderboardmessage) {
            Ok(received) => {
                if received < 1 {
                    println!("Perte de la connexion avec le serveur");
                    return;
                }
            }
            Err(_) => {
                println!("Perte de la connexion avec le serveur");
                return;
            }
        }
        let publicleaderboardmessage_string = str::from_utf8(&publicleaderboardmessage).unwrap();
        println!("{}", publicleaderboardmessage_string);

        let mut anymessage : String = "".to_owned();
        let challengemessage_string = "";

        while !anymessage.contains("RoundSummary") {
            if anymessage.contains("MD5HashCash") {
                
                println!("Challenge MD5HashCash: {}", &anymessage);
    
                let md5challengejson : MD5ChallengeStruct = serde_json::from_str(&anymessage).unwrap();
                
                let md5challengestructinput = md5challengejson.Challenge.MD5HashCash;
                
                let md5hashcashchallenge : MD5HashCashChallenge = MD5HashCashChallenge::new(md5challengestructinput);
            
                let md5hashcashchallengeoutput : MD5HashCashOutput = md5hashcashchallenge.solve();
            
                println!("{:?}",md5hashcashchallengeoutput);
            
                let md5challengejsonresponse = MD5ChallengeStructResponse{ChallengeResult:{MD5ChallengeEnumResponse{answer:MD5ChallengeEnumResponse2{MD5HashCash:md5hashcashchallengeoutput},next_target:"petit_test".to_string()}}};
                sendData(serde_json::to_string(&md5challengejsonresponse).unwrap(),&stream);
            }
            else if anymessage.contains("MonstrousMaze") {
                println!("Challenge MonstrousMaze: {}", &anymessage);

                let monstrousmazejson : MonstrousMazeChallengeStruct = serde_json::from_str(&anymessage).unwrap();
                
                let monstrousmazechallengestructinput = monstrousmazejson.Challenge.MonstrousMaze;
                
                let monstrousmazechallenge : MonstrousMazeChallenge = MonstrousMazeChallenge::new(monstrousmazechallengestructinput);
            
                let monstrousmazechallengeoutput : MonstrousMazeOutput = monstrousmazechallenge.solve();
            
                println!("{:?}",monstrousmazechallengeoutput);

                let monstrousmazejsonresponse = MonstrousMazeStructResponse{ChallengeResult:{MonstrousMazeEnumResponse{answer:MonstrousMazeEnumResponse2{MonstrousMaze:monstrousmazechallengeoutput},next_target:"petit_test".to_string()}}};
                sendData(serde_json::to_string(&monstrousmazejsonresponse).unwrap(),&stream);
    
            }
            anymessage= "".to_owned();  

            let mut resultmessagesize = &mut [0u8;4];
        
            match stream.read(resultmessagesize) {
                Ok(received) => {
                    if received < 1 {
                        println!("Perte de la connexion avec le serveur");
                        return;
                    }
                }
                Err(_) => {
                    println!("Perte de la connexion avec le serveur");
                    return;
                }
            }
            println!("Read message size : {:?}", u32::from_be_bytes([resultmessagesize[0],resultmessagesize[1],resultmessagesize[2],resultmessagesize[3]]));

            let mut resultmessage = vec! [0u8;usize::from_be_bytes([0,0,0,0,resultmessagesize[0],resultmessagesize[1],resultmessagesize[2],resultmessagesize[3]])];
            
            match stream.read(&mut resultmessage) {
                Ok(received) => {
                    if received < 1 {
                        println!("Perte de la connexion avec le serveur");
                        return;
                    }
                }
                Err(_) => {
                    println!("Perte de la connexion avec le serveur");
                    return;
                }
            }
            anymessage = str::from_utf8(&resultmessage).unwrap().to_string();
            println!("Any Message : {:}",anymessage);

            if (anymessage.contains("EndOfGame")) {
                println!("End Of Game");
                break;
            }
        }
    }
}

pub fn main() {
    println!("Tentative de connexion au serveur...");
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            println!("Connexion au serveur réussie !");
            exchange_with_server(stream);
            
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}